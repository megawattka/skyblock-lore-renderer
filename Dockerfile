# Этап 1: Сборка
FROM rust:1.91-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Этап 2: Минимальный образ для запуска
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/skyblock-lore-renderer .
EXPOSE 8080
CMD ["./skyblock-lore-renderer"]