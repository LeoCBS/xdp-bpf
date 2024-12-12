program_name=udpredirect
udpproducer_name=udpproducer

all: run

build:
	docker build --target builder -t $(program_name) .

image: image-udpproducer
	docker build -t $(program_name) .

image-udpproducer:
	docker build -f Dockerfile.udpproducer -t $(udpproducer_name) .

run-standalone: image
	docker run --privileged -ti --init  --rm -v debugfs:/sys/kernel/debug:rw  $(program_name)

run: stop image
	docker-compose run --rm udpproducer

stop:
	docker-compose down -v --remove-orphans
