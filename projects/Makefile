PROJECTS = $(shell ls -l | grep '^d.*$$' | awk '{print $$NF}')

all:
	for i in $(PROJECTS); do \
		make -C $$i; \
	done

clean:
	for i in $(PROJECTS); do \
		make clean -C $$i; \
	done

# Usage : $ make project NAME=<project name>
project:
	cargo new $(NAME)
	cp ProjectMakefileTemplate $(NAME)/Makefile

# Usage : $ make library NAME=<library name>
library:
	cargo new --lib $(NAME)
	cp ProjectMakefileTemplate $(NAME)/Makefile

rebuild:
	for i in $(PROJECTS); do \
		make rebuild -C $$i; \
	done

