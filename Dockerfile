# syntax=docker/dockerfile:1.4

FROM node:20-bullseye as builder

    WORKDIR /app

    # Install rust
    RUN \
        curl --proto '=https' --tlsv1.2 -sSf --output /tmp/rustup https://sh.rustup.rs && \
        chmod +x /tmp/rustup && \
        bash /tmp/rustup -y
    ENV PATH="$PATH:/root/.cargo/bin"
    RUN rustup target add wasm32-unknown-unknown

    # Install Cargo B(inary)Install
    RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

    RUN cargo binstall --no-confirm trunk

    COPY package.json package-lock.yaml ./

    RUN npm ci

    COPY . .

    RUN trunk build --release

FROM nginxinc/nginx-unprivileged:1.25-alpine

    EXPOSE 8000

    COPY --from=builder --link /app/dist/ /usr/share/nginx/html/

    COPY ./nginx.conf /etc/nginx/conf.d/default.conf
