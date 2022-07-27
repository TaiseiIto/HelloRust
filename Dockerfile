FROM alpine

RUN apk update
# gcc, git, ld, make, etc.
RUN apk add --no-cache alpine-sdk
# git setting
RUN git config --global pull.rebase false
# gpg
RUN apk add --no-cache gnupg
# rust
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
# ssh
RUN apk add --no-cache openssh
RUN mkdir /root/.ssh
# set time zone UTC+9 (Japan)
RUN apk add --no-cache tzdata
RUN cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
RUN apk del tzdata
# editor
RUN apk add --no-cache vim

# clone this repository
WORKDIR /root
RUN git clone https://github.com/TaiseiIto/HelloRust.git
WORKDIR HelloRust

# ash setting
ENV ENV="/root/.profile"
RUN cp ash/.profile "$ENV"

# vim setting
RUN cp vim/.vimrc ..

