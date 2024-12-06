fn main() {
    env_logger::init();
    udpredirect::load_bpf_redirect_to_xdp_queue();
}
