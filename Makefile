program_name=xdp-udp

all: run

build:
	docker build --target build -t $(program_name) .

image:
	docker build --target image -t $(program_name) .

xdp-drop.o:
	clang -Wall -target bpf -c $(program_name).c -o $(program_name).o

run: image
	docker run --privileged -ti --rm --name $(program_name) $(program_name)

reqs:
	sudo apt-get install -y make gcc libssl-dev bc libelf-dev libcap-dev \
  clang gcc-multilib llvm libncurses5-dev git pkg-config libmnl-dev bison flex \
  graphviz

clean:
	docker rmi $(program_name)
	docker system prune -f

