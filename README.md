# Noisebell Client Template

This is a template to subscribe to and listen to webhook events for [Noisebell](https://github.com/jetpham/Noisebell)

## Features

- Automatic server registration with retry mechanism
- Comprehensive logging
- Automatic port discovery

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- For cross-compilation:
  - `cross` tool: `cargo install cross`
  - Docker (required for cross-compilation)

## Configuration

The client can be configured using environment variables:

- `SERVER_URL`: The URL of the server to register with (default: `http://127.0.0.1:8080`)
> This is still a work in progress template and project overall, this is because I deploy the webhook server and the listening client on the same device, this will not always be true

## Usage

### Local Development

1. Clone the template:
```bash
git clone <repository-url>
cd <repository-name>
```

2. Build the project:
```bash
cargo build --release
```

3. Run the client:
```bash
# Using default server URL
cargo run

# Or specify a custom server URL
SERVER_URL=http://your-server:8080 cargo run
```

### Cross-compilation for Raspberry Pi

1. Install the cross-compilation tool:
```bash
cargo install cross
```

2. Build for Raspberry Pi (aarch64):
```bash
cross build --release --target aarch64-unknown-linux-gnu
```

3. Copy the binary to your Raspberry Pi:
```bash
# Replace with your Raspberry Pi's hostname or IP
scp target/aarch64-unknown-linux-gnu/release/noisebell-client-template noisebridge@noisebell.local:~/noisebell-client-template
```

4. On the Raspberry Pi, make the binary executable and run it:
```bash
# SSH into your Raspberry Pi first
ssh noisebridge@noisebell.local

# Make the binary executable
chmod +x ~/noisebell-client-template

# Run with custom server URL (if needed)
SERVER_URL=http://your-server:8080 ./noisebell-client-template
```

## Webhook Payload Format

The client expects webhook payloads in the following JSON format:

```json
{
    "event_type": "string",
    "timestamp": "string",
    "new_state": "open|closed"
}
```

## Circuit States

The client manages two circuit states:
- `Open`: Indicates no noise detected
- `Closed`: Indicates noise detected

## Logging

Logs are written to both stdout and daily rotating files for the past 7 days in the `logs` directory

## Customization

Instead of printing that something happened, do something super cool!

## Contributing

Feel free to add to this template, but this is more a starting off point to make your own projects