FROM debian:bookworm-slim

WORKDIR /app

## copy the main binary
COPY ./main ./main
COPY ./site ./target/site

RUN apt update && apt install curl -y

EXPOSE 3000

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

HEALTHCHECK --interval=10s --start-period=20s CMD curl -f http://localhost:3000 || exit 1
RUN cd ./target/site && ls
CMD ./main
