[![Crates.io Version](https://img.shields.io/crates/v/nadeo-api)](https://crates.io/crates/nadeo-api)
[![Crates.io License](https://img.shields.io/crates/l/nadeo-api)](https://github.com/TgZ39/nadeo-api/blob/master/LICENSE)
[![docs.rs](https://img.shields.io/docsrs/nadeo-api)](https://docs.rs/nadeo-api/)

# nadeo-api

This library handles all the **authentication** for working with the [Nadeo API](https://webservices.openplanet.dev/).

## Installation

Via command line:

```sh
cargo add nadeo-api
```

Via `Cargo.toml`:

```toml
nadeo-api = "0.3.0"
```
## Getting started

Creating a client:

```rust
use nadeo_api::NadeoClient;

let mut client = NadeoClient::builder()
    .with_normal_auth("my_email", "my_password")
    .with_server_auth("my_username", "my_other_password")
    .with_oauth("my_identifier", "my_secret")
    .user_agent("My cool Application / my.email@domain.com")
    .build()
    .await?;
```

Creating a request:

```rust
use nadeo_api::NadeoRequest;
use nadeo_api::auth::AuthType;
use nadeo_api::request::Method;

let request = NadeoRequest::builder()
    .url("api_endpoint_url")
    .auth_type(AuthType::NadeoServices)
    .method(Method::GET)
    .body("some text/json") // optional
    .build()?;
```

Executing a request:

```rust
let mut client = /* snap */;
let request = /* snap */;

let response = client.execute(request).await?;
```


## License

This project is licensed under [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).

