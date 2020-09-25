#
# BUILD SERVER
#
FROM rust:1.46 as build
WORKDIR /wordfun

# Precompile our dependencies: this speeds up subsequent builds
ENV USER=root
RUN cargo init
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release
RUN rm -fr src

# Build for real this time
COPY src src
RUN cargo build --release

#
# BUILD WEB ASSETS
#
FROM node as jsbuild
RUN npm i -g parcel
WORKDIR /wordfun
COPY package.json .
COPY package-lock.json .
RUN npm install
COPY .parcelrc .
COPY .postcssrc .
ARG COMMIT_ID
COPY web web
RUN npm run build

#
# BUILD FINAL IMAGE
#
# We use buster-slim. I'd like to use alpine, but something didn't work right with static compilation, and I didn't feel
# like debugging it.
#
FROM debian:buster-slim

WORKDIR /wordfun
COPY data data
COPY --from=jsbuild  /wordfun/dist dist
COPY --from=build /wordfun/target/release/wordfun /usr/bin/wordfun

ARG COMMIT_ID
ENV COMMIT_ID=$COMMIT_ID
ENV RUST_LOG=warn
ENV ASSETS_DIR=dist

EXPOSE 3000

CMD ["wordfun"]
