programm_name=xdp-udp

all: run

build:
	docker build --target build -t $(programm_name) .

image:
	docker build --target image -t $(programm_name) .

xdp-drop.o:
	clang -Wall -target bpf -c $(programm_name).c -o $(programm_name).o

run: image
	docker run --privileged -ti --rm --name $(programm_name) $(programm_name)

reqs:
	sudo apt-get install -y make gcc libssl-dev bc libelf-dev libcap-dev \
  clang gcc-multilib llvm libncurses5-dev git pkg-config libmnl-dev bison flex \
  graphviz

clean:
	docker rmi $(programm_name)
	docker system prune -f

