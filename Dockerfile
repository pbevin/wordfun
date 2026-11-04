#
# BUILD SERVER
#
FROM rust:1-alpine AS build
WORKDIR /wordfun
RUN apk add --no-cache musl-dev

# Precompile our dependencies: this speeds up subsequent builds
ENV USER=root
RUN cargo init --name wordfun
COPY api/Cargo.toml api/Cargo.lock ./
RUN cargo build --release --locked && rm -fr src

# Build for real this time (touch so cargo notices the new sources)
COPY api/src src
RUN touch src/main.rs src/lib.rs && cargo build --release --locked

#
# BUILD WEB ASSETS
#
FROM node:22-alpine AS jsbuild
WORKDIR /wordfun
COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile
COPY tsconfig.json .
COPY src src
COPY public public
RUN yarn build

#
# BUILD FINAL IMAGE
#
FROM alpine:3

WORKDIR /wordfun
COPY api/data data
RUN LANG=C sort data/lexicon.txt data/extra-words -o data/lexicon.txt
COPY --from=jsbuild /wordfun/build build
COPY --from=build /wordfun/target/release/wordfun /usr/bin/wordfun

ARG SOURCE_COMMIT
ENV SOURCE_COMMIT=$SOURCE_COMMIT
ENV RUST_LOG=info

EXPOSE 3000

CMD ["wordfun", "--server-port", "3000", "--bind", "0.0.0.0", "--assets-dir", "build"]
