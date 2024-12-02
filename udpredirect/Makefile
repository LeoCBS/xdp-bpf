program_name=udpredirect

all: run

build:
	docker build --target build -t $(program_name) .

run: build
	docker run --privileged -ti --rm $(program_name) /app/${program_name}
