FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/card_game
COPY . .

RUN cd card_game_front_end && trunk build --release
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/card_game/target/release/card_game_backend /usr/local/bin/card_game_backend
COPY --from=build /usr/src/card_game/card_game_front_end/dist /usr/local/bin/dist

WORKDIR /usr/local/bin

CMD ["card_game_backend"]