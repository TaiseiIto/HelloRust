COMPILER = rustc
REMOVE = rm
SOURCE = $(shell ls *.rs)
TARGET = $(SOURCE.rs:)

all: $(TARGET)

clean:
	$(REMOVE) $(TARGET)

rebuild: clean
	make

$(TARGET): $(SOURCE)
	$(COMPILER) $(SOURCE)

