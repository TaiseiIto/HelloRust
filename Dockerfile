FROM ubuntu

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update
# gcc, ld, make, etc.
RUN apt install build-essential -y
# git
RUN apt install git -y
RUN apt install git-email -y
RUN git config --global pull.rebase false
RUN mkdir /root/.ssh
# rust
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
# tmux
RUN apt install tmux -y
# tzdata
RUN apt install tzdata -y
RUN cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
# editor
RUN apt install vim -y

# clone this repository
WORKDIR /root
RUN git clone https://github.com/TaiseiIto/HelloRust.git
WORKDIR HelloRust

# ash setting
RUN cat ash/.profile >> /root/.bashrc

# tmux setting
RUN cp tmux/.tmux.conf ..

# vim setting
RUN cat vim/.vimrc >> ../.vimrc

