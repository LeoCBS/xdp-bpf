program_name=udpredirect

all: run

build:
	docker build -D --target build -t $(program_name) .

run: build
	docker run --privileged -ti --rm $(program_name) ./target/release/${program_name}
