FROM ubuntu:14.04
MAINTAINER Tomas Korcak "korczis@gmail.com"

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install \
       build-essential \
       ca-certificates \
       curl \
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

ENV RUST_ARCHIVE=rust-1.9.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

COPY . .

RUN ldconfig -vv

# Add to PATH - /root/.cargo/bin

# Install dependencies
RUN make install_deps

# Build all gooddata-fs stuff
RUN make all

RUN mkdir -p /gd-fs

CMD ["sudo", "./bin/gooddata-fs", "tomas.korcak+gem_tester@gooddata.com", "jindrisska", "/gd-fs"]
