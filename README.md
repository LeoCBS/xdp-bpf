# XDP and BPF study case

This project was created to practice and study AF_XBF and BPF approach

## How to run source code and discard UDP packages using BPF

Build and run an ubuntu docker container:

    make run

Start UDP server:

    nohup ./start-udp-server.sh &

Check if python UDP server starting without errors

    cat nohup.out

Send one UDP message

    nc -u 172.17.0.2 1025 

Write something and check UDP server response

```
root@16ed8a46e4e6:/# nc -u 127.0.0.1 1025
lala
Your data was 5 bytes long
```
Load BPF program using iproute2 command

    ip link set dev eth0 xdp obj /xdp-udp.o sec drop_udp

Check if your bpf program was attached

    ip link show dev eth0

output:

```
root@7f25eeaff4c3:/# ip link show dev eth0
65: eth0@if66: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 xdp qdisc noqueue state UP mode DEFAULT group default 
    link/ether 02:42:ac:11:00:02 brd ff:ff:ff:ff:ff:ff link-netnsid 0
    prog/xdp id 133 tag 649153f4d32aeb29 jited
```

## AF_XDP redirect UDP packages example using Rust

In this example we will create a Rust program to attach a BPF program to eth0 interface to redirect UDP packages
to XDP sockets (the RX ring and the TX ring). After attach BPF program, it will pool messages from XDP socket
using xsk-rs (A Rust interface for Linux AF_XDP sockets using libxdp)

See diagram below to check more details:





## Useful commands

Inspect all UDP packages

    sudo tcpdump -i lo -n udp port 2399 -v -X

Cat BPF trace/prints on host or mount debugFS on docker container

    sudo cat  /sys/kernel/debug/tracing/trace_pipe


## References

https://gitlab.com/mwiget/xdp-drop-test/-/tree/master?ref_type=heads
https://github.com/xdp-project/xdp-tutorial/blob/master/tracing03-xdp-debug-print/README.org
https://docs.kernel.org/bpf/map_xskmap.html
https://github.com/DouglasGray/xsk-rs
https://github.com/libbpf/libbpf-rs/tree/master/examples
https://gist.github.com/fntlnz/f6638d59e0e39f0993219684d9bf57d3
https://docs.kernel.org/networking/af_xdp.html
https://hemslo.io/run-ebpf-programs-in-docker-using-docker-bpf/
