# syntax=docker/dockerfile:1

FROM node:22-bookworm-slim AS frontend-builder
WORKDIR /app/frontend

COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

FROM rust:1.94-bookworm AS backend-builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime
WORKDIR /app

ENV HOST=0.0.0.0
ENV PORT=8080
ENV FRONTEND_DIST=frontend/dist

COPY --from=backend-builder /app/target/release/ipocrate /usr/local/bin/ipocrate
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist

EXPOSE 8080

ENTRYPOINT ["ipocrate"]
