PROJECT = $(shell pwd | awk -F '/' '{print $$NF}')
DEBUG = target/debug/$(PROJECT)
RELEASE = target/release/$(PROJECT)
SOURCE = $(shell find src | grep '^.*\.rs$$')
PORT = 7878

all: $(RELEASE)
	./client.sh $(PROJECT) $(PORT) > clientout.txt &
	./target/release/$(PROJECT) $(PORT) > serverout.txt
	cat serverout.txt

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

