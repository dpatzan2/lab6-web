# Usa la imagen oficial de Rust como base
FROM rust:1.67 AS builder

WORKDIR /app

# Copia los archivos de proyecto
COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src

# Compila el proyecto en modo release
RUN cargo build --release

# Crea una imagen liviana para producción
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copia el binario desde la etapa de compilación
COPY --from=builder /app/target/release/laliga-tracker-backend /usr/local/bin/laliga-tracker-backend

# Variable de entorno para conectar a MongoDB (puedes definirla en tu docker-compose)
ENV MONGO_URI=mongodb://host.docker.internal:27017

EXPOSE 8080

CMD ["laliga-tracker-backend"]