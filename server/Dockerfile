# Use the rust:latest Docker image as the base image
FROM rust:latest as builder

# Create a new directory for the application
WORKDIR /app

# Copy the source code into the container
#COPY /Users/sinembilgeguler/Desktop/coap-project /app
#COPY ./src/main.rs /usr/src/myapp/main.rs
#COPY ./Cargo.toml /app/
#COPY ./src/main.rs .
COPY ./Cargo.toml .
COPY ./src/main.rs /app/src/main.rs

# Build the Rust application inside the container
RUN cargo build --release

RUN apt-get update && apt-get install -y tcpdump


# Build the runtime image
FROM rust:slim-buster

# Create a new directory for the application
WORKDIR /app

COPY --from=builder /app/target/release/coap-server .

# Expose the default CoAP port
EXPOSE 5683/udp

# Run the Rust application when the container starts
CMD ["/app/coap-server"]

