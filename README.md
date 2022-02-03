# OpenAPI v3 ![gh actions](https://github.com/JoelEllis/openapiv31/actions/workflows/rust.yml/badge.svg)

**NOTE: This is an incomplete implementation, and a work in progress. It does not follow 3.1 fully compliantly yet.**

This crate provides data structures that represent the [OpenAPI v3.1.x specification](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.1.0.md). It uses the [Schemars](https://github.com/GREsau/schemars) crate for defining JSON schemas.

## Example

```rust
use serde_json;
use openapiv31::OpenAPI;

fn main() {
    let data = include_str!("openapi.json");
    let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
    println!("{:?}", openapi);
}
```

## Limitations

Schemas without a type will end up as any data type as per the specification and can have any parameters of any schema type. Some Open API documents don't include the type parameter, so it would be nice to try to derive the type.

## Similar Crates

* [openapiv3](https://github.com/glademiller/openapiv3)
* [openapi](https://crates.io/crates/openapi)
* [rweb-openapi](https://lib.rs/crates/rweb-openapi)

## License

This crate is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
