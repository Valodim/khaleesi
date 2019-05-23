FROM buildpack-deps:buster

# Some of the dependencies I need to build a few libraries,
# personalize to your needs. You can use multi-stage builds
# to produce a lightweight image.
#RUN apt-get update && \
#    apt-get install -y curl file gcc g++ git make openssh-client \
#    autoconf automake cmake libtool libcurl4-openssl-dev libssl-dev \
#    libelf-dev libdw-dev binutils-dev zlib1g-dev libiberty-dev wget \
#    xz-utils pkg-config python

RUN apt-get update && \
    apt-get install -y libical-dev clang

#ENV KCOV_VERSION 33
#RUN wget https://github.com/SimonKagstrom/kcov/archive/v$KCOV_VERSION.tar.gz && \
#    tar xzf v$KCOV_VERSION.tar.gz && \
#    rm v$KCOV_VERSION.tar.gz && \
#    cd kcov-$KCOV_VERSION && \
#    mkdir build && cd build && \
#    cmake .. && make && make install && \
#    cd ../.. && rm -rf kcov-$KCOV_VERSION

#RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

#ENV PATH "$PATH:/root/.cargo/bin"
#ENV RUSTFLAGS "-C link-dead-code"
#ENV CFG_RELEASE_CHANNEL "nightly"

#RUN rustup update && \
#    rustup install nightly && \
#    rustup default nightly

#ENV RUSTFMT_VERSION 0.3.1
#RUN wget https://github.com/rust-lang-nursery/rustfmt/archive/${RUSTFMT_VERSION}.tar.gz && \
#    tar xzf ${RUSTFMT_VERSION}.tar.gz && rm ${RUSTFMT_VERSION}.tar.gz && \
#    cd rustfmt-${RUSTFMT_VERSION} && \
#    $HOME/.cargo/bin/cargo install --path . && \
#    cd .. && rm -rf rustfmt-${RUSTFMT_VERSION}

#RUN bash -l -c 'echo $(rustc --print sysroot)/lib >> /etc/ld.so.conf'
#RUN bash -l -c 'echo /usr/local/lib >> /etc/ld.so.conf'
#RUN ldconfig
#EXEC bash
