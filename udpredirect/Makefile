program_name=udpredirect

all: run

build:
	docker build -D --target builder -t $(program_name) .

image:
	docker build -D -t $(program_name) .

run: image
	docker run --privileged -ti --init --rm -v debugfs:/sys/kernel/debug:rw  $(program_name)
