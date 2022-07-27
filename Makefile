DOCKER = docker
DOCKER_IMAGE = hello-rust
DOCKER_IMAGE_TAG = latest
DOCKER_CONTAINER = hello-rust
GENIMG = genimg/genimg

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

clean-devenv:
	$(SCRIPT_PREFIX)script$(DELIMITER)clean-devenv$(SCRIPT_SUFFIX) $(DOCKER) $(DOCKER_IMAGE) $(DOCKER_CONTAINER)

devenv:
	$(SCRIPT_PREFIX)script$(DELIMITER)devenv$(SCRIPT_SUFFIX) $(DOCKER) $(DOCKER_IMAGE) $(DOCKER_IMAGE_TAG) $(DOCKER_CONTAINER)

# Only the developer can execute it.
# usage : $ make gitconfig KEY=<GitHub private key path> GPG=<.gnupg path>
gitconfig:
	$(DOCKER) cp $(KEY) $(DOCKER_CONTAINER):/root/HelloRust/ssh/github && \
	$(DOCKER) cp $(GPG) $(DOCKER_CONTAINER):/root/.gnupg && \
	make docker-start && \
	$(DOCKER) exec -it $(DOCKER_CONTAINER) /root/HelloRust/git/gitconfig.sh

rebuild-devenv: clean-devenv
	make devenv

.PHONY: clean-devenv devenv rebuild-devenv

