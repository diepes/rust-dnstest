# https://github.com/GoogleContainerTools/distroless/tree/main
#
FROM rust:latest as build-env
WORKDIR /app
COPY . /app
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/dnstest /
ENTRYPOINT ["/dnstest"]
CMD ["google.com"]
