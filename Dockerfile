FROM rustlang/rust:nightly-alpine

WORKDIR /app

## copy the main binary
# COPY ./main ./main
# COPY ./target/site ./target/site

RUN apk update && apk add curl

EXPOSE 3000

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

HEALTHCHECK --interval=10s --start-period=20s CMD curl -f http://localhost:3000 || exit 1

CMD ./main
