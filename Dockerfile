FROM codercom/code-server

FROM ekidd/rust-musl-builder:nightly

COPY --from=0 /usr/local/bin/code-server /usr/local/bin/

USER root

ENV ROCKET_ADDRESS 0.0.0.0

RUN apt-get update \
 && apt-get install -y software-properties-common \
 && add-apt-repository ppa:ubuntu-toolchain-r/test \
 && apt-get update \
 && apt-get upgrade -y \ 
  # upgrade libstdc++6 since code-server was buid in ubuntu 18.10 and rust-musl-builder uses 16.04
  libstdc++6 \ 
 && apt-get install -y \
  # build dependencies
  software-properties-common \
  gcc-4.9 \
  unzip \ 
  wget \
  expect \
  # code-server dependencies
  openssl \
  net-tools

USER rust

RUN rustup update
RUN rustup default nightly

RUN rustup component add rls-preview rust-analysis rust-src

# https://marketplace.visualstudio.com/_apis/public/gallery/publishers/rust-lang/vsextensions/rust/0.5.3/vspackage
ADD extensions/rust-lang.rust-0.5.3.vsix rust-lang.rust-0.5.3.vsix.zip
RUN mkdir -p /home/rust/.code-server/extensions
RUN unzip rust-lang.rust-0.5.3.vsix.zip 'extension/*' -d /home/rust/.code-server/extensions/ \
 && mv /home/rust/.code-server/extensions/extension /home/rust/.code-server/extensions/rust-lang.rust-0.5.3

RUN cargo install \
 cargo-watch \
 cargo-add

RUN mkdir src \
 && touch src/lib.rs

ADD . ./
ENV CARGO_TARGET_DIR=/home/rust/targetcache
RUN cargo build
RUN cargo build --tests

ADD rls-build.sh ./
RUN ./rls-build.sh
