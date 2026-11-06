# Wordfun

This is the code that runs [wordfun.ca](https://www.wordfun.ca/).

## Building with Docker

The simplest way to build and run this is with Docker:

```bash
docker build -t wordfun .
docker run -it -p3000:3000 wordfun
```

Then open a browser at http://localhost:3000/

## Building for Development

The app comes in two parts: a server, written in Rust, which handles searching, and a client, written in TypeScript and
React (Create React App).

You need a current stable Rust ([rustup](https://rustup.rs/)) and Node.js 22 or later with Yarn 1.

I run the client and server in separate windows.

- For the server, `cd api && cargo run`. It listens on 127.0.0.1:4000 and must be run from the `api` directory so it
  can find `data/`. The first build takes a few minutes; later ones take seconds.

- For the client, run `yarn install` the first time, and then `yarn start`. This opens a dev server on
  [localhost:3000](http://localhost:3000) with hot reloading, and proxies API calls to the Rust server on port 4000.

Tests: `cd api && cargo test`. GitHub Actions runs the Rust tests and the client build on every push to `main`.

## Deploying

The live site runs on [Fly.io](https://fly.io/) as a single Docker container, configured in `fly.toml`.

One-time setup:

```bash
brew install flyctl          # or: curl -L https://fly.io/install.sh | sh
fly auth login
fly launch --no-deploy --copy-config   # creates the app; pick a new name if "wordfun" is taken
```

To deploy the current checkout:

```bash
fly deploy --build-arg SOURCE_COMMIT=$(git rev-parse HEAD)
```

`SOURCE_COMMIT` is reported at `/version.txt`, so you can check which commit is live. To add words, append them to
`api/data/extra-words` (the Docker build merges and sorts them into the lexicon) and deploy.

The domain `wordfun.ca` is registered and its DNS hosted at Hover. To point it at Fly:

```bash
fly certs add www.wordfun.ca
fly certs add wordfun.ca
fly ips list                 # gives the IPv4 and IPv6 addresses
```

Then in Hover set `www` to a CNAME of `<app-name>.fly.dev`, and the apex `wordfun.ca` to A and AAAA records with the
addresses from `fly ips list`. `fly certs show wordfun.ca` confirms when the certificates are issued.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

- Please format your code with `cargo fmt` and `prettier`.
- Please make sure tests run with `cargo test`.

## License

[MIT](https://choosealicense.com/licenses/mit/)
