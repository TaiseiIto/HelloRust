PROJECT = $(shell pwd | awk -F '/' '{print $$NF}')
DEBUG = target/debug/$(PROJECT)
RELEASE = target/release/$(PROJECT)
SOURCE = $(shell find src | grep '^.*\.rs$$')
GREP_TARGET = poem.txt
QUERIES = frog body monomorphization

all: $(RELEASE)
	for q in $(QUERIES); do ./target/release/$(PROJECT) $$q $(GREP_TARGET); done

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

