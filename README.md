# Noisebell Client Template

This is a template to subscribe to and listen to webhook events for [Noisebell](https://github.com/jetpham/Noisebell)

## Features

- Automatic server registration with retry mechanism
- Comprehensive logging with daily rotating files
- Automatic port discovery
- Webhook event handling
- Health and status monitoring

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- For cross-compilation:
  - `cross` tool: `cargo install cross`
  - Docker (required for cross-compilation)

## Configuration

The client can be configured using environment variables:

- `SERVER_URL`: The URL of the server to register with (required)

> Note: This template is designed to be deployed on the same device as the webhook server, but can be modified for different deployment scenarios.

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
# Server URL is required
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

# Run with server URL
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

## Logging

Logs are written to both stdout and daily rotating files in the `logs` directory. The logging system uses the `tracing` crate and includes:

- Console output for development
- Daily rotating log files
- Different log levels (info, error, etc.)

## Customization

The template is designed to be a starting point for your own projects. You can:

1. Modify the webhook handling logic in `src/lib.rs`
2. Add new endpoints in the Axum router
3. Implement custom state management
4. Add additional monitoring or notification features

## Contributing

Feel free to fork this template and customize it for your own projects. This is meant to be a starting point that you can build upon.

## License

This project is open source and available under the MIT License.
