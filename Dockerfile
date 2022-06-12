FROM rust:latest as builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/rust_app
COPY . .

RUN cd card_game_front_end && trunk build --release
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/rust_app/target/release/card_game_backend /usr/local/bin/card_game_backend

COPY --from=build /usr/src/rust_app/card_game_front_end/dist /usr/local/bin/dist

WORKDIR /usr/local/bin

CMD ["sudo" ,"card_game_backend"]