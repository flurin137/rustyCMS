FROM rust:latest AS build
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/rusty_cms .
USER 1000
EXPOSE 5050
CMD ["./rusty_cms"]