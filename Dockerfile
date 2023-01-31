FROM rust:1.67 as build
WORKDIR /usr/src/itemis
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=build /usr/local/cargo/bin/itemis /usr/local/bin/itemis
CMD ["itemis", "-i"]
