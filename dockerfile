FROM rust:1.78 AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/infs803_group7_backend /usr/local/bin/infs803_group7_backend
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["infs803_group7_backend"]