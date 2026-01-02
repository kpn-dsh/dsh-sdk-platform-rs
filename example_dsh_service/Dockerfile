### builder
FROM rust:latest AS builder

#cache 
VOLUME /home/dsh/target

WORKDIR /home/dsh
COPY . .

# install dependencies (CMake required for rdkafka)
RUN apt-get update && apt-get install -y \
    cmake 

# Build the Rust service
RUN cargo install --path .

### runner
FROM debian:trixie-slim

# args
ARG UID
RUN test -n "$UID" || (echo "UID argument not provided" && false)
ARG GID
RUN test -n "$GID" || (echo "GID argument not provided" && false)
ARG USER=dsh
ARG GROUP=dsh
ARG WORKDIR=/home/${USER}


RUN apt-get update && apt-get -y upgrade

# Add user to group
RUN groupadd --gid ${GID} ${GROUP}
RUN useradd -ms /bin/bash --uid ${UID} --gid ${GID} ${USER}

# Copy binary from builder (make sure the name is same as the one in Cargo.toml)
COPY --from=builder /usr/local/cargo/bin/dsh-rs-example /home/dsh/bin/dsh-rs-example

# Expose port for metrics
EXPOSE 8080

# Set the entry point or command to run the Rust application
ENTRYPOINT [ "/home/dsh/bin/dsh-rs-example" ]