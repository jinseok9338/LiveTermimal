FROM rust:latest as build


RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
RUN apt-get update && apt-get install nodejs -y
RUN apt-get install npm -y
RUN npm i tailwindcss -g

WORKDIR /usr/src/card_game
COPY . .

RUN cd front_end && npm run tailwind-build && trunk build --release

FROM nginx:latest

COPY --from=build /usr/src/card_game/front_end/dist /usr/share/nginx/html

EXPOSE 80

# Start Nginx when the container has launched
CMD ["nginx", "-g", "daemon off;"]





