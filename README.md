# faas-rust-template

![Docker](https://github.com/0x0177b11f/faas-rust-template/workflows/Docker/badge.svg) [![Docker Repository on Quay](https://quay.io/repository/0x0177b11f/faas-rust-template/status "Docker Repository on Quay")](https://quay.io/repository/0x0177b11f/faas-rust-template)

An OpenFaaS of-watchdog template written in Rust.

## Features

- use rust 1.41.1
- use hyper 0.13
- use async function
- enable all tokio features
- enable lto

## Use template

```sh
$ faas template pull https://github.com/0x0177b11f/faas-rust-template
$ faas new --list
Languages available as templates:
- rust
```

## Function

```Rust
use hyper::{Body, Request, Response};
use std::convert::Infallible;

const PHRASE: &str = "Hello, World!";

pub async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(PHRASE.into()))
}

// Returns:
// Hello, World!
```
