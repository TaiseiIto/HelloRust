PROJECT = $(shell pwd | awk -F '/' '{print $$NF}')
DEBUG = target/debug/$(PROJECT)
RELEASE = target/release/$(PROJECT)
SOURCE = $(shell find src | grep '^.*\.rs$$')

all: $(RELEASE)

build: $(DEBUG)

check:
	cargo check

run:
	cargo run

$(RELEASE): $(SOURCE)
	cargo build --release


$(DEBUG): $(SOURCE)
	cargo build

