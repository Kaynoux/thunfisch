FROM rust:1.94-bullseye AS builder

# Basic toolchain
RUN apt update -y && apt install -y curl wget tar

WORKDIR /root

RUN mkdir -p /usr/local/src/

# Fastchess
RUN wget https://github.com/Disservin/fastchess/releases/download/v1.8.0-alpha/fastchess-linux-arm64.tar
RUN tar -xf fastchess-linux-arm64.tar -C /usr/local/src


# Runtime stage
FROM ubuntu:24.04

# Minimal runtime dependencies only
RUN apt update -y && apt install -y curl git

WORKDIR /root
RUN mkdir -p /__w
RUN mkdir -p /usr/local/src/

# Copy pre-built artifacts from builder stage
COPY --from=builder /usr/local/src/fastchess-linux-arm64 /usr/local/src/fastchess-linux-arm64
ENV PATH="/usr/local/src/fastchess-linux-arm64:$PATH"
