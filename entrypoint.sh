#!/bin/sh

set -e # exit script in case of errors

ip addr show dev eth0

GW=$(ip route | grep default | cut -d' ' -f3)
echo ""
echo "-------------------------------------------------------------"
echo "Without XDP drop app installed, udp client and server works on $GW ..."

nohup ./start-udp-server.sh 0<&- &> udp-server.log.file &

cat udp-server.log.file
echo "deu bom"
