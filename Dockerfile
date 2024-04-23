FROM ubuntu:23.04 as builder
LABEL Description="Build environment"
ENV USERNAME=developer

# Install steps
RUN apt-get update -y && apt-get upgrade -y
RUN apt-get update -y && apt-get install -y --no-install-recommends apt-utils \
                                                                    build-essential \ 
                                                                    sudo \
                                                                    git \
                                                                    git-core \
                                                                    bash-completion \
                                                                    ssh \
                                                                    cmake \
                                                                    cmake-curses-gui \
                                                                    tig \
                                                                    clang \
                                                                    libclang-rt-15-dev \
                                                                    net-tools \
                                                                    less \
                                                                    ca-certificates \
                                                                    vim \
                                                                    curl \
                                                                    libprotobuf-dev \
    && apt-get clean all

RUN useradd -m $USERNAME && echo "$USERNAME:$USERNAME" | chpasswd && adduser $USERNAME sudo
USER $USERNAME
WORKDIR /home/$USERNAME

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/home/$USERNAME/.cargo/bin:${PATH}"

# Create .bashrc and create development directory in home
COPY .devcontainer/.bashrc /tmp/.bashrc
RUN cat /tmp/.bashrc > .bashrc
RUN mkdir development
RUN cargo install --force cbindgen