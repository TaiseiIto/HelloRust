PROJECTS = $(shell ls -l | grep '^d.*$$' | awk '{print $$NF}')

all:
	for i in $(PROJECTS); do \
		make -C $$i; \
	done

clean:
	for i in $(PROJECTS); do \
		make clean -C $$i; \
	done

rebuild:
	for i in $(PROJECTS); do \
		make rebuild -C $$i; \
	done

