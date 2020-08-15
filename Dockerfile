FROM rust:latest
LABEL Name=rusty_cms Version=0.0.1
WORKDIR /rusty_cms
COPY . .
RUN cargo install --path .
EXPOSE 7878
CMD ["rusty_cms"]