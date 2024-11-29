use anyhow::{bail, Context};
use libbpf_rs::skel::SkelBuilder;

pub fn load_bpf_redirect_to_xdp_queue() -> anyhow::Result<()> {
    //let ifindex = get_ifindex(&config.interface.if_name).context("could not get interface")?;
    /*let mut skel_builder = MdSkelBuilder::default();
    skel_builder.obj_builder.debug(true);

    // Open, parse bpf objects and set constants
    let mut open_object = MaybeUninit::uninit();
    let open_skel = skel_builder.open(&mut open_object)?;

    // Load into kernel
    let mut skel = open_skel.load()?;*/

    println!("deu bom");
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
