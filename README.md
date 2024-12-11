# Doip Codec

The `Doip Codec` crate provides a `DoipCodec` implementation for encoding and decoding Diagnostics Over Internet Protocol (DoIP) messages. It is designed to integrate seamlessly with the Tokio ecosystem, leveraging `tokio-util`'s `Framed` for efficient stream-based reading and writing of DoIP messages.

## Features

- Full support for encoding and decoding DoIP messages as per the ISO 13400 standard.
- Easy integration with Tokio's asynchronous I/O framework.
- Robust and efficient handling of DoIP message framing.
- Customizable for various DoIP use cases, including vehicle diagnostics and testing.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
doip-codec = "1.0.0"
```

Then, include the crate in your code:

```rust
use doip_codec::DoipCodec;
```

## Usage

Here's a simple example to get started with `DoipCodec`:

```rust
use doip::{
    header::{
        payload::vehicle_identification_request::VehicleIdentificationRequest, version::DoipVersion,
    },
    message::message::DoipMessage,
};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a DoIP server
    let stream = TcpStream::connect("127.0.0.1:13400").await?;

    // Wrap the stream with the DoipCodec
    let mut framed = Framed::new(stream, DoipCodec);

    // Send a DoIP message
    let request = DoipMessage::new(
        DoipVersion::Iso13400_2012,
        Box::new(VehicleIdentificationRequest {}),
    ); // Example payload

    framed.send(request).await?;

    // Receive a DoIP message
    if let Some(response) = framed.next().await {
        match response {
            Ok(msg) => println!("Received message: {:?}", msg),
            Err(e) => eprintln!("Failed to decode message: {}", e),
        }
    }

    Ok(())
}
```

## Documentation

Comprehensive API documentation is available on [docs.rs](https://docs.rs/doip-codec/).

## Why DoIP?

Diagnostics Over Internet Protocol (DoIP) is a modern diagnostic communication protocol that leverages IP-based networks for vehicle diagnostics, making it a critical component in automotive software. The `Doip Codec` crate simplifies the implementation of DoIP messaging for Rust developers.

## Contributing

Contributions are welcome! Feel free to open issues, submit pull requests, or suggest features. Please follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) when contributing.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
