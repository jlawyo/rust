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
- `/echo` endpoint
- `/hello` endpoint
- `PUT` and `GET /file/{filename}` store and retrieve text files

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
- Refer to rust.yml for more information

## File API

Files are stored in the `files/` directory. `PUT` saves the request body as a file, and `GET` returns its contents. Use a single filename in the URL, not a directory path.

Save a file:

```bash
curl -X PUT http://127.0.0.1:3000/file/file.txt \
  -H "Content-Type: text/plain" \
  --data "Hello, file!"
```

Retrieve it:

```bash
curl http://127.0.0.1:3000/file/file.txt
```

Sending another `PUT` to the same URL replaces the file's contents.

## Project Status

This is a starter project in a working state. It is a foundation for learning Rust web development or expanding into a larger application.

## License

Copyright (c) <2026> < jlawyo and christiantherobinson-source>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
