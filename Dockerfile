# Builder
FROM rust:latest as builder
WORKDIR /srv

COPY . ./codebase
RUN cd ./codebase && cargo build --release
RUN cp ./codebase/target/release/portfolio-vico ./server
RUN rm -r ./codebase

# Runner
FROM debian
WORKDIR /
COPY --from=builder /srv .

CMD ["/server"]

