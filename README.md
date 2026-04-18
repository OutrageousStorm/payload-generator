# ⚡ Android A/B Update Payload Generator

Rust CLI tool to generate Android A/B update payloads from system images. Used in custom ROM distribution.

## Features

- Fast payload generation (Rust/native performance)
- Supports sparse image compression
- Validates checksums
- Generates delta payloads between versions
- Full metadata signing

## Install

```bash
cargo install --path .
payload-gen --help
```

## Usage

```bash
# Generate full update payload
payload-gen --system system.img --boot boot.img --output update.zip

# Generate delta payload (smaller size)
payload-gen --old-system old_system.img --new-system new_system.img --delta

# Sign payload
payload-gen --input update.zip --sign-with key.pem
```

## Android Update Format

A/B payloads follow Google's OTA specification:
- **Payload**: serialized protobuf with delta/full operations
- **Metadata**: operation list + checksum
- **Signature**: RSA-2048 SHA-256

This tool handles all three.

---

*Used by: LineageOS, CalyxOS, custom ROM builders*
