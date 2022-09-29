FROM ubuntu:latest
MAINTAINER Matthew Brown <matt@ederoyd.co.uk>
ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt upgrade -y 

RUN apt install -y \
    make \
    musl \
    musl-dev \
    direnv \
    unzip \
    jq \
    zip

RUN apt install -y \
    git-extras \
    fish
    # pipx \
    # musl-tools \
    # cmake \
    # g++ \
    # xvfb \
    # libatk1.0-0 \
    # libatk-bridge2.0-0 \
    # librust-gdk-pixbuf-sys-dev \
    # libgtk-3-0 \
    # libgbm1 \
    # libasound2 \
    # libnss3 \
    # oathtool \
