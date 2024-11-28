#!/bin/sh

set -e # exit script in case of errors

echo "starting nc udp server on "

#nc -u -l 2399

python3 udpserver.py