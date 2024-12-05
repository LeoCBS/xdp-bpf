use anyhow::{bail, Context};
use libbpf_rs::skel::OpenSkel;
use libbpf_rs::skel::SkelBuilder;
use libbpf_rs::MapCore;
use libbpf_rs::MapFlags;
use std::mem::MaybeUninit;

mod xdpmd {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bpf/udp.skel.rs"));
}
use xdpmd::*;
mod xdpqueue;

pub fn load_bpf_redirect_to_xdp_queue() -> anyhow::Result<()> {
    let if_name = String::from("eth0");
    let ifindex = get_ifindex(&if_name).context("could not get interface")?;
    let mut skel_builder = UdpSkelBuilder::default();
    skel_builder.obj_builder.debug(true);

    // Open, parse bpf objects and set constants
    let mut open_object = MaybeUninit::uninit();
    let open_skel = skel_builder.open(&mut open_object)?;

    // Load into kernel
    let mut skel = open_skel.load()?;

    let queue_id: u32 = 0;
    let mut qr = xdpqueue::Reader::new(queue_id)?;

    // update xsk_map with queue_id and sock_fd
    let sock_fd = qr.fd();
    skel.maps.xsks_map.update(
        &queue_id.to_ne_bytes(),
        &sock_fd.to_ne_bytes(),
        MapFlags::empty(),
    )?;

    // attach prog to interface and run
    let link = skel.progs.udp_capture.attach_xdp(ifindex as i32)?;
    skel.links = UdpLinks {
        udp_capture: Some(link),
    };

    println!("deu bom {ifindex}");

    if let Err(err) = qr.run() {
        log::error!("error to run queue {}", err);
    }

    println!("deu bom {ifindex}");
    Ok(())
}

fn get_ifindex(name: impl AsRef<str>) -> anyhow::Result<u32> {
    let name_cstr = std::ffi::CString::new(name.as_ref()).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid interface name")
    })?;
    match unsafe { libc::if_nametoindex(name_cstr.as_ptr()) } {
        0 => bail!("if_nametoindex failed: {}", std::io::Error::last_os_error()),
        n => Ok(n),
    }
}
