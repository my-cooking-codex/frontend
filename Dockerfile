# syntax=docker/dockerfile:1.4
FROM rust:1-buster as builder

    WORKDIR /app

    ENV PNPM_HOME="/root/.local/share/pnpm"
    ENV PATH="$PNPM_HOME:$PATH"

    RUN rustup target add wasm32-unknown-unknown

    RUN wget -qO- https://get.pnpm.io/install.sh | SHELL=bash sh - \
        && pnpm add -g node-linux-x64

    RUN cargo install trunk

    COPY package.json pnpm-lock.yaml ./

    RUN pnpm install

    COPY . .

    RUN trunk build --release

FROM nginxinc/nginx-unprivileged:stable-alpine

    EXPOSE 8000

    COPY --from=builder --link /app/dist/ /usr/share/nginx/html/

    COPY ./nginx.conf /etc/nginx/conf.d/default.conf
