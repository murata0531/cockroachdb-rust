FROM rust:1.60.0-slim-buster

# ローカルのtargetディレクトリにビルドするとマウントしている時に遅くなるのでビルドディレクトリを変える
ENV CARGO_TARGET_DIR=/tmp/target \
    DEBIAN_FRONTEND=noninteractive \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8

RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     locales \
     libpq-dev \
     gnupg \
     apt-transport-https\
     libssl-dev \
     pkg-config \
     curl \
     build-essential \
     git \
     wget \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen \
  && echo "install rust tools" \
  && cargo install cargo-watch cargo-make \
  && cargo install diesel_cli --no-default-features --features postgres

RUN USER=root cargo new --bin app
WORKDIR /app
COPY ./Cargo.* ./
RUN cargo build --color never \
  && rm src/*.rs

COPY . /app
RUN cargo build

CMD ["cargo", "run"]