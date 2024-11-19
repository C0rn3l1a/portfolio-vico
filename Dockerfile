# Builder
FROM rust:latest as builder
WORKDIR /srv

COPY . ./codebase
RUN cd ./codebase && cargo build --release
RUN cp ./codebase/target/release/portfolio-vico ./server
RUN cp -a ./codebase/static/. ./static
RUN cp -a ./codebase/templates/. ./templates
RUN rm -r ./codebase

# Runner
FROM debian
WORKDIR /
COPY --from=builder /srv .

CMD ["/server"]

