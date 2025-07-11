# CDN CLI

A command-line tool for managing files on a CDN (Content Delivery Network) using S3/R2 buckets as storage backends.

By default, when uploading, the path is set to a random UUID unless specified with the `--path` option.

## Features
- Upload files to S3/R2 buckets
- List files on the CDN
- Delete files from the CDN
- Register new CDN endpoints

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.60+ recommended)

### Install via Cargo
you can install the CLI directly from the latest source using Cargo:
```sh
cargo install --git https://github.com/moddedTechnic/cdn-cli
```
This will build and install the `cdn` binary to your Cargo bin directory (usually `~/.cargo/bin`).

### Build from Source
Alternatively, you can build manually:
```sh
git clone https://github.com/moddedTechnic/cdn-cli.git
cd cdn_cli
cargo build --release
```
The compiled binary will be in `target/release/cdn`.

## Usage

Run the CLI with:
```sh
cdn [--config CONFIG] <COMMAND> [OPTIONS]
```

### Example Commands
- Register an R2 bucket:
  ```sh
  cdn register r2 <DOMAIN> <ACCOUNT_ID> <BUCKET_NAME> [--index] [--default]
  ```
- Unregister a domain:
  ```sh
  cdn unregister <DOMAIN>
  ```
- Upload a file (random UUID path by default):
  ```sh
  cdn upload [--target DOMAIN] [--mime MIME] [--path PATH] [--password] <FILE>
  ```
  ```sh
  cdn list buckets
  ```

## Configuration
Configuration is managed via a config file, which defaults to `~/.cdncli`. See `src/config.rs` for details.

## Contributing
Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](LICENSE)
