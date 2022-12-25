PROJECT = $(shell pwd | awk -F '/' '{print $$NF}')
DEBUG = target/debug/$(PROJECT)
RELEASE = target/release/$(PROJECT)
SOURCE = $(shell find src | grep '^.*\.rs$$')

all: $(RELEASE)
	./target/release/$(PROJECT) test poem.txt

build: $(DEBUG)

check:
	cargo check

clean:
	rm $(DEBUG) $(RELEASE)

rebuild: clean
	make

run:
	cargo run

$(RELEASE): $(SOURCE)
	cargo build --release


$(DEBUG): $(SOURCE)
	cargo build

