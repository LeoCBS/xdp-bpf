use std::os::fd::AsRawFd;
use std::os::fd::RawFd;

use anyhow::{self, Context};
use xsk_rs::{
    config::{Interface, LibxdpFlags, SocketConfig, UmemConfig},
    FillQueue, FrameDesc, RxQueue, Socket, Umem,
};

#[derive(Debug)]
pub struct Reader {
    queue_id: u32,
    umem: Umem,
    frames: Vec<FrameDesc>,
    rx: RxQueue,
    fq: FillQueue,
    poll_ms_timeout: i32,
}

impl Reader {
    pub fn new(queue_id: u32) -> anyhow::Result<Reader> {
        // The size of a umem frame (cannot be smaller than 2048 and must be a power of two)
        let frame_size: u32 = 4096;
        let fill_queue_size = 2048;

        let umem_config = UmemConfig::builder()
            .frame_size(frame_size.try_into()?)
            .fill_queue_size(fill_queue_size.try_into()?)
            .build()
            .context("invalid umem config")?;

        let frame_count = 2048;
        let (umem, frames) = Umem::new(umem_config, frame_count.try_into()?, false)
            .context("failed to create UMEM")?;

        let if_name = String::from("eth0")
            .parse::<Interface>()
            .context("could not get interface")?;
        let rx_queue_size = 2048;

        let socket_config = SocketConfig::builder()
            .rx_queue_size(rx_queue_size.try_into()?)
            .libxdp_flags(LibxdpFlags::XSK_LIBXDP_FLAGS_INHIBIT_PROG_LOAD)
            .xdp_flags(xsk_rs::config::XdpFlags::XDP_FLAGS_SKB_MODE)
            .bind_flags(xsk_rs::config::BindFlags::XDP_COPY)
            .build();

        let (_tx, rx, fq_and_cq) = unsafe { Socket::new(socket_config, &umem, &if_name, queue_id) }
            .context("failed to create socket")?;

        let (mut fq, _cq) = fq_and_cq.context("missing fill queue and comp queue")?;

        unsafe {
            fq.produce(&frames);
        }

        Ok(Reader {
            queue_id,
            umem,
            frames,
            rx,
            fq,
            poll_ms_timeout: 100,
        })
    }

    pub fn fd(&self) -> RawFd {
        self.rx.fd().as_raw_fd()
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            match unsafe {
                self.rx
                    .poll_and_consume(&mut self.frames, self.poll_ms_timeout)
                    .context("failed to poll and consume")?
            } {
                0 => {
                    // No frames consumed, wake up fill queue if required
                    if self.fq.needs_wakeup() {
                        log::info!("waking up receiver fill queue");
                        let fd = self.rx.fd_mut();
                        self.fq
                            .wakeup(fd, self.poll_ms_timeout)
                            .context("failed to wake up")?;
                    }
                }
                frames_rcvd => {
                    log::info!("receiver rx queue consumed {} frames", frames_rcvd);
                    for recv_desc in self.frames.iter().take(frames_rcvd) {
                        let data = unsafe { self.umem.data(recv_desc) };
                        log::info!("received this packet {data:?}");
                    }
                    // Add frames back to fill queue
                    while unsafe {
                        let fd = self.rx.fd_mut();
                        self.fq
                            .produce_and_wakeup(
                                &self.frames[..frames_rcvd],
                                fd,
                                self.poll_ms_timeout,
                            )
                            .context("failed to produce and wake up")?
                    } != frames_rcvd
                    {
                        // Loop until frames added to the fill ring.
                        log::info!("receiver fill queue failed to allocate");
                    }
                    log::info!("submitted {} frames to receiver fill queue", frames_rcvd);
                }
            }
        }
    }
}
