FROM public.ecr.aws/docker/library/rust:1.67

# Setting up work directory
WORKDIR /hello

# Copying the source code
COPY ./hello/src /hello/src
COPY ./hello/Cargo.toml /hello/Cargo.toml
COPY ./hello/Cargo.lock /hello/Cargo.lock

# Building the source code
RUN cargo build --release

# Exposing the binary
CMD ["/hello/target/release/hello"]