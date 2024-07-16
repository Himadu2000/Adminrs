FROM rustlang/rust:nightly-alpine

WORKDIR /app

## copy the main binary
COPY ./main ./main
COPY ./target/site ./target/site

RUN apt update && apt install curl -y

EXPOSE 3000

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

HEALTHCHECK --interval=10s --start-period=20s CMD curl -f http://localhost:3000 || exit 1

CMD ./main

# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/leptos_start /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

EXPOSE $PORT
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/leptos_start"]
