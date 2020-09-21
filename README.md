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

The app comes in two parts: a server, written in Rust, which handles searching, and a client, written in JavaScript and
React.

For the server, you will need the Rust environment, and for the client, you will need Node.js and npm. I don't have any
advice on minimum versions; at the time of writing, I am using Rust 1.46, Node.js 14.11.0, and npm 6.14.7, which are all
the most current releases. In general, I like to keep tools up to date, so it's not a bad bet to just get the latest
version of everything.

- [Installing Rust](http://rustup.sh/)
- [Installing Node.js and npm](https://nodejs.org/en/download/)

I run the client and server in separate windows.

- For the server, run `cargo run`. This will take a couple of minutes the first time, because there are a lot of
  dependencies to compile. Subsequent compiles will take just a few seconds.

- For the client, run `npm install` the first time, and then `npm start` to run the server.

The server listens on port 3000, but for the best development experience, open a browser at
[localhost:1234][http://localhost:1234]. That way, you get hot module replacement and don't have to keep reloading the
page.

If you have problems with any of the above, please open a GitHub issue.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

- Please format your code with `cargo fmt` and `prettier`.
- Please make sure tests run with `cargo test`.

## License

[MIT](https://choosealicense.com/licenses/mit/)
