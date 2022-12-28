PROJECT = $(shell pwd | awk -F '/' '{print $$NF}')
DEBUG = target/debug/$(PROJECT)
RELEASE = target/release/$(PROJECT)
SOURCE = $(shell find src | grep '^.*\.rs$$')

all: $(RELEASE) test

build: $(DEBUG)

check:
	cargo check

clean:
	rm $(DEBUG) $(RELEASE)

rebuild: clean
	make

run:
	cargo run

test:
	cargo test

$(RELEASE): $(SOURCE)
	cargo build --release


$(DEBUG): $(SOURCE)
	cargo build

