# hello_cargo

A small Rust web service built with Axum and Tokio. It exposes a simple `/hello` endpoint and serves as a lightweight starter project for learning how to build async HTTP servers in Rust.

## Overview

This project demonstrates the basics of a Rust web app:

- async server startup with Tokio
- routing and request handling with Axum
- JSON responses using `serde_json`
- a simple integration-style test for the API

The application currently listens on port `3000` and responds with a JSON greeting payload.

## Features

- Minimal HTTP server setup
- Single `/hello` endpoint
- JSON response payload
- Automated test coverage for the route

## Installation

Make sure Rust and Cargo are installed on your machine:

```bash
rustc --version
cargo --version
```

Then clone the repository and build the project:

```bash
git clone <repository-url>
cd rust
cargo build
```

## Usage

Start the server:

```bash
cargo run
```

Once it is running, send a request:

```bash
curl http://127.0.0.1:3000/hello
```

Example response:

```json
{
  "message": "Hello, World!"
}
```

## Building

Compile the project without running it:

```bash
cargo build
```

Build an optimized release binary:

```bash
cargo build --release
```

## Testing

Run the test suite:

```bash
cargo test
```

The test confirms that the `/hello` route returns the expected JSON response.

## Project Structure

```text
.
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   └── main.rs
└── target/
```

- `src/main.rs` starts the server and binds it to `127.0.0.1:3000`
- `src/lib.rs` defines the router and the `/hello` handler
- `Cargo.toml` contains the Rust package metadata and dependencies

## API

### GET /hello

Returns a JSON greeting.

Example:

```bash
curl -i http://127.0.0.1:3000/hello
```

Response:

```http
HTTP/1.1 200 OK
content-type: application/json

{"message":"Hello, World!"}
```

## Contributing

Contributions are welcome. If you would like to improve the project:

1. Fork the repository
2. Create a branch for your feature or fix
3. Make your changes
4. Run `cargo test`
5. Open a pull request with a summary of the update

## License

This project does not currently include a license file. If you plan to share or distribute it publicly, add a license such as MIT or Apache 2.0.

## Project Status

This is a starter project in a working state. It is a foundation for learning Rust web development or expanding into a larger application.
