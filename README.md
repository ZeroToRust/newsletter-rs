# Email Newsletter Service

A fast, simple, and scalable email newsletter service for blogs, built in Rust. This project provides an efficient way to manage and send blog newsletters, ensuring reliability and performance.

## Features

- **Fast & Scalable:** Handles large mailing lists efficiently with Rust’s performance.  
- **Easy-to-Use API:** RESTful endpoints for managing subscribers and sending emails.  
- **Secure & Configurable:** Implements best practices for authentication and customization.  

## Installation

### Prerequisites

Ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [PostgreSQL](https://www.postgresql.org/) (for storing subscriber data)

### Steps

1. **Clone the Repository**
   ```sh
   git clone https://github.com/ZeroToRust/newsletter-rs.git
   cd newsletter-rs
   ```

2. **Install Dependencies**
   ```sh
   cargo build
   ```
   
## Troubleshooting  
- **Database Connection Issues:**  
  - Ensure PostgreSQL is installed and running.  

- **Cargo Errors:**  
  - Run `cargo clean` followed by `cargo build` to resolve potential dependency issues.  
  - Ensure you are using the latest Rust and Cargo versions (`rustup update`).  

If you encounter issues, feel free to open an issue in the repository.


