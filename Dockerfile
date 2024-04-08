FROM rustlang/rust:nightly-alpine

RUN rustup target add wasm32-unknown-unknown
RUN apk add --no-cache musl-dev
RUN cargo install --locked trunk

WORKDIR /app

COPY . .

RUN trunk build --release

EXPOSE 8080

CMD ["trunk", "serve", "--release"]