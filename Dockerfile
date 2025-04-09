FROM rust:alpine as build
RUN cargo new sciigilo
WORKDIR /sciigilo
COPY Cargo.toml Cargo.lock ./
RUN apk add --no-cache musl-dev
RUN cargo build --release
RUN rm -rf src/*
COPY ./ ./
RUN rm -f target/release/deps/sciigilo*
RUN cargo verify-project && cargo build --release

FROM alpine:latest
WORKDIR /sciigilo
COPY --from=build /sciigilo/target/release/sciigilo .
ENV PORT 5000
EXPOSE ${PORT}
CMD [ "./sciigilo" ]
