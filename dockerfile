FROM rust:bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim as runner
RUN apt-get update && apt install -y openssl ca-certificates
COPY --from=builder /usr/local/cargo/bin/infs803_group7_backend /usr/local/bin/infs803_group7_backend
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["infs803_group7_backend"]