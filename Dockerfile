FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/fresh-post /app/fresh-post
COPY --from=builder /app/web /app/web
EXPOSE 8080
ENV DATA_DIR=/data
CMD ["/app/fresh-post"]
