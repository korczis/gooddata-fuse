FROM ubuntu:14.04
MAINTAINER Tomas Korcak "korczis@gmail.com"

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install \
       build-essential \
       ca-certificates \
       curl \
       fuse \
       gcc \
       graphviz \
       libc6-dev \
       libfuse-dev \
       libssl-dev \
       pkg-config \
       upx \
       -qqy \
       --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir /rust
WORKDIR /rust

RUN sudo curl -sSf https://static.rust-lang.org/rustup.sh | sudo sh -s -- -y --verbose

COPY Makefile .

RUN ldconfig -vv

# Add to PATH - /root/.cargo/bin

# Install dependencies
RUN make install_deps

COPY . .

# Build all gooddata-fs stuff
RUN make all

RUN mkdir -p /gd-fs

ENTRYPOINT ["./scripts/docker/gooddata-fs-wrapper.sh"]
