
# Email Newsletter Service

A fast, simple, and scalable email newsletter service for blogs, built in Rust. This project provides an efficient way to manage and send blog newsletters.

## Features

- **Fast & Scalable:** Handles large mailing lists efficiently with Rust’s performance.  
- **Easy-to-Use API:** RESTful endpoints for managing subscribers and sending emails.  
- **Secure & Configurable:** Implements best practices for authentication and customization.  

## Installation

### Prerequisites

Ensure you have the following installed on your system:

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [PostgreSQL](https://www.postgresql.org/) (Docker image recommended)

### Steps

1. **Clone the Repository**
   ```sh
   git clone https://github.com/ZeroToRust/newsletter-rs.git
   cd newsletter-rs
   ```

2. **Build**
   ```sh
   cargo build
   ```
## Testing database setup

**testcontainers** is a Rust library for running Docker containers in your tests (e.g. databases, services) to test real infrastructure.

**testcontainers-modules** provides ready-to-use containers (like PostgreSQL, Redis, Kafka) built on top of **testcontainers**, so you don't have to configure them manually.

We use [`testcontainers`](https://crates.io/crates/testcontainers) and [`testcontainers-modules`](https://crates.io/crates/testcontainers-modules) to run PostgreSQL in Docker for integration tests.

### Features

- Dynamic connection string per test run
- Explicit container tag (`postgres:17.5`) for reproducibility
- Compatible with both local runs and GitHub Actions

### Running Tests

```sh
cargo test --all
```
   
## Troubleshooting  
- **Database Connection Issues:**  
  - Ensure PostgreSQL is installed and running.  

- **Cargo Errors:**  
  - Run `cargo clean` followed by `cargo build` to resolve potential dependency issues.  
  - Ensure you are using the latest Rust and Cargo versions (`rustup update`).  
- **Linking Issues:**  
  - Ensure you have installed `mold` (Linux) or `lld` (macOS).  
  - Refer to the [Build & Link Optimization Guide](./docs/build_optimization.md) for troubleshooting.  
  
