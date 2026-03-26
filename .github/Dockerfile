FROM rust:1.94-bullseye

## Basic toolchain
RUN apt update -y && apt install -y curl wget tar git

WORKDIR /root

## Required Directories
RUN mkdir -p /__w
RUN mkdir -p /usr/local/src/

## Install Fastchess
RUN wget https://github.com/Disservin/fastchess/releases/download/v1.8.0-alpha/fastchess-linux-arm64.tar
RUN tar -xf fastchess-linux-arm64.tar -C /usr/local/src

ENV PATH="/usr/local/src/fastchess-linux-arm64:/usr/local/cargo:$PATH"
