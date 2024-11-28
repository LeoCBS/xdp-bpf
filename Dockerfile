ARG PROGRAM_NAME="xdp-udp"

FROM ubuntu:20.04 AS build

ARG PROGRAM_NAME

RUN export DEBIAN_FRONTEND=noninteractive && apt-get update \
  && apt-get install -y clang llvm netcat python3

COPY *.c /
RUN find /usr -name types.h
RUN echo ${PROGRAM_NAME}
RUN clang -g -c -O2 -target bpf -I/usr/include/x86_64-linux-gnu/ -c ${PROGRAM_NAME}.c -o ${PROGRAM_NAME}.o \
  && objdump -t ${PROGRAM_NAME}.o && llvm-objdump -S ${PROGRAM_NAME}.o


FROM ubuntu:20.04 AS image

ARG PROGRAM_NAME

RUN export DEBIAN_FRONTEND=noninteractive \
  && apt-get update && apt-get install -y iproute2 iputils-ping netcat python3

COPY --from=build /${PROGRAM_NAME}.o /
COPY /entrypoint.sh /
COPY ./start-udp-server.sh /
COPY ./udpserver.py /
COPY ./udp-send-msg.py /

ENTRYPOINT ["bash"]
