DOCKER = docker
DOCKER_IMAGE = hello-rust
DOCKER_IMAGE_TAG = latest
DOCKER_CONTAINER = hello-rust

ifeq ($(OS), Windows_NT)
BLANK =
DELIMITER = \$(BLANK)
SCRIPT_PREFIX =
SCRIPT_SUFFIX = .bat
else
DELIMITER = /
SCRIPT_PREFIX = ./
SCRIPT_SUFFIX = .sh
endif

all:
	make -C projects

clean:
	make clean -C projects

clean-devenv:
	$(SCRIPT_PREFIX)script$(DELIMITER)clean-devenv$(SCRIPT_SUFFIX) $(DOCKER) $(DOCKER_IMAGE) $(DOCKER_CONTAINER)

devenv:
	$(SCRIPT_PREFIX)script$(DELIMITER)devenv$(SCRIPT_SUFFIX) $(DOCKER) $(DOCKER_IMAGE) $(DOCKER_IMAGE_TAG) $(DOCKER_CONTAINER)

# Only the developer can execute it.
# usage : $ make gitconfig KEY=<GitHub private key path> GPG=<.gnupg path> CRATESIO=<crates.io API token path>
gitconfig:
	$(DOCKER) cp $(KEY) $(DOCKER_CONTAINER):/root/HelloRust/ssh/github && \
	$(DOCKER) cp $(GPG) $(DOCKER_CONTAINER):/root/.gnupg && \
	$(DOCKER) cp $(CRATESIO) $(DOCKER_CONTAINER):/root/HelloRust/crates.io/key && \
	$(DOCKER) start $(DOCKER_CONTAINER)
	$(DOCKER) exec -it $(DOCKER_CONTAINER) /root/HelloRust/git/gitconfig.sh
	$(DOCKER) stop $(DOCKER_CONTAINER)

rebuild:
	make rebuild -C projects

rebuild-devenv: clean-devenv
	make devenv

.PHONY: clean-devenv devenv rebuild-devenv

