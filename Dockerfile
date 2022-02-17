#
# BUILD SERVER
#
FROM rust:1.51-alpine as build
WORKDIR /wordfun
RUN apk add musl-dev

# Precompile our dependencies: this speeds up subsequent builds
ENV USER=root
RUN cargo init
COPY api/Cargo.toml .
COPY api/Cargo.lock .
RUN cargo build --release
RUN rm -fr src

# Build for real this time
COPY api/src src
RUN cargo build --release

#
# BUILD WEB ASSETS
#
FROM node:16 as jsbuild
RUN npm i -g parcel
WORKDIR /wordfun
COPY /package.json .
COPY /yarn.lock .
RUN yarn
COPY /tsconfig.json .
COPY /src src
COPY /public public
RUN npx browserslist --update-db
RUN npm run build

#
# BUILD FINAL IMAGE
#
FROM alpine

WORKDIR /wordfun
COPY /api/data data
RUN LANG=c sort data/lexicon.txt data/extra-words -o data/lexicon.txt
COPY --from=jsbuild  /wordfun/build build
COPY --from=build /wordfun/target/release/wordfun /usr/bin/wordfun

ARG SOURCE_COMMIT
ENV SOURCE_COMMIT=$SOURCE_COMMIT
ENV RUST_LOG=info

EXPOSE 3000

CMD ["wordfun", "--server-port", "3000", "--bind", "0.0.0.0", "--assets-dir", "build"]
