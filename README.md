# opensea-stream

This is a fork of the opensea-stream library by wanderers-nft. Since the library seems abandoned, I reimplemented the websocket connection and tried to fix all bugs. If you find more or have feature request, feel free to open an Issue or Pull Request.

## Installation

add to your `Cargo.toml`:

```
[dependencies]
opensea-stream = { git = "https://github.com/joergkiesewetter/opensea-stream-rs.git", tag = "0.4.0" }
```

## Example

The easiest way to connect to opensea is shown in the example at `examples/measure_events.rs`. To execute it, run the following command:

```
cargo run --example measure_events -- <your_api_key>
```
