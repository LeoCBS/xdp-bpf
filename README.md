# XDP and BPF study case

This project was created to practice and study AF_XBF and BPF approach

## How to run source code and discard UDP packages using BPF

    nohup ./start-udp-server.sh &

Check if python UDP server starting without errors

    cat nohup.out

Send one UDP message

    nc -u 127.0.0.1 1025 

Write something and check UDP server response

```
root@16ed8a46e4e6:/# nc -u 127.0.0.1 1025
lala
Your data was 5 bytes long
```
Load BPF program using iproute2 command

    ip link set dev eth0 xdp obj /xdp-udp.o sec drop_udp


## References

https://gitlab.com/mwiget/xdp-drop-test/-/tree/master?ref_type=heads
