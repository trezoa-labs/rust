FROM ubuntu:22.04
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    gcc libc6-dev ca-certificates bzip2 \
    libssl-dev pkg-config

ENV RUSTUP_INIT_SKIP_PATH_CHECK="yes"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -v --no-modify-path
RUN cp ${HOME}/.cargo/bin/* /usr/local/bin/

RUN cargo install --git https://github.com/trezoa-xyz/cargo-run-trezoa-tests.git \
    --rev ccfb0a3f5f967f3cf52fef0c95e283c7cab1836e \
    --bin cargo-run-trezoa-tests --root /usr/local

RUN mkdir -p /tmp/.cache/trezoa/v1.51/platform-tools
RUN curl -L -o platform-tools-linux-x86_64.tar.bz2 https://github.com/trezoa-xyz/platform-tools/releases/download/v1.51/platform-tools-linux-x86_64.tar.bz2
RUN tar -xjf platform-tools-linux-x86_64.tar.bz2 --strip-components 1 -C /tmp/.cache/trezoa/v1.51/platform-tools
RUN rustup toolchain link trezoa /tmp/.cache/trezoa/v1.51/platform-tools/rust
RUN cp -R ${HOME}/.rustup /tmp/

ENV CARGO_TARGET_SBPFV3_TREZOA_TREZOA_RUNNER="cargo-run-trezoa-tests --heap-size 104857600"
ENV LLVM_HOME="/tmp/.cache/trezoa/v1.51/platform-tools/llvm"
ENV CC="/tmp/.cache/trezoa/v1.51/platform-tools/llvm/bin/clang"
ENV RUSTUP_TOOLCHAIN="trezoa"
