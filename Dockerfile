FROM rust:bullseye

COPY . .
RUN apt update -yqq && apt install -yqq --no-install-recommends nodejs libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev patchelf librsvg2-dev
RUN curl -f https://get.pnpm.io/v6.14.js | node - add --global pnpm@6
RUN pnpm config set store-dir .pnpm-store
