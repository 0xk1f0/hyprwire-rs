# hyprwire-rs</h1>

A hyprwire Rust implementation

## Motivation

Based on the [initial implementation](https://github.com/hyprwm/hyprwire) in C++

Since I needed a Rust compatible library for my wallpaper tool [rwpspread](https://github.com/0xk1f0/rwpspread), I decided to put this into a standalone package.

## Getting Started

```rust
use hyprwire_rs::client::HyprWireClient;
use hyprwire_rs::wire;

// connect to a socket
let socket_path = "/tmp/some-socket.sock";
let client: HyprWireClient = HyprWireClient::connect(&socket_path).expect("could not connect");

// perform a handshake
let supported_version: u32 = 1;
let protocols: Vec<wire::Protocol> = client.perform_handshake(supported_version).expect("could not perform handshake");

// bind to a protocol
let some_proto: wire::Protocol = protocols.first().expect("no protocols returned")
let object_id: u32 = client.bind_protocol(&some_proto.spec).expect("could not bind to protocol");

// send a message
client.send_message(
    wire::Code::HW_GENERIC_PROTOCOL_MESSAGE,
    &[
        wire::Value::Object(object_id),
        wire::Value::Uint(0),
        wire::Value::Seq(client.get_sequence()),
    ],
).expcet("could not send message");

// read a message
let response = client.read_message().expect("could not read message");

// disconnect
client.disconnect().expect("could not disconnect properly");
```
