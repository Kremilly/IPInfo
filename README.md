# IPInfo

Empower your IP data retrieval tasks by leveraging Rust's capabilities to effortlessly extract comprehensive IP information from URLs, enhancing efficiency and precision in data processing workflows.

To Install using [crates.io](https://crates.io)

```shell
cargo install ipinfo-cli
```

Basic usage:

```shell
ipfinfo example.com
```

Dependencies:

```toml
ipgeolocate = "0.3.6"
regex = "1.10.4"
tokio = { version = "1.37.0", features = ["full"] }
```
