[![Crates.io Version](https://img.shields.io/crates/v/nadeo-api)](https://crates.io/crates/nadeo-api)
[![Crates.io License](https://img.shields.io/crates/l/nadeo-api)](./LICENSE)
[![docs.rs](https://img.shields.io/docsrs/nadeo-api)](https://docs.rs/nadeo-api/)

---

About
---
This library provides an interface (or whatever you want to call it) for working with the [Nadeo API](https://webservices.openplanet.dev/). It handles **authentication** automatically but API requests have to be build up manually by the user.

⚠️ This project is in early development ⚠️

Installation
---

Run
```sh
cargo add nadeo-api
```

or add this line to your `Cargo.toml` with the desired version:
```toml
nadeo-api = "0.2.2"
```

Getting started
---

Creating a client:

```rust
use nadeo_api::NadeoClient;

let mut client = NadeoClient::builder()
    .with_normal_auth("my_email", "my_password")
    .with_oauth("my_identifier", "my_secret")
    .user_agent("My cool Application / my.email@domain.com")
    .build()
    .await?;
```

Creating a request:

```rust
use nadeo_api::NadeoRequest;
use nadeo_api::auth::AuthType;
use nadeo_api::request::HttpMethod;

let request = NadeoRequest::builder()
    .url("ape_endpoint_url")
    .auth_type(AuthType::NadeoServices)
    .method(HttpMethod::Get)
    .build()?;
```

Executing a request:

```rust
let mut client = //snap;
let mut request = //snap;

let response = client.execute(request).await?;
```

License
---

This project is licensed under [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).