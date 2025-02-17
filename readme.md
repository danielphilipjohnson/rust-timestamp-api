# 🚀 Timestamp Microservice
> A high-performance REST API for date-time operations and timezone conversions built with Rust and Actix-web

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Actix](https://img.shields.io/badge/Actix-000000?style=for-the-badge&logo=rust&logoColor=white)

## 🌟 Features
✔ **Current time retrieval in UTC and Unix timestamp**  
✔ **Date parsing and timezone conversions**  
✔ **Weekday calculations for any date**  
✔ **Support for all major timezone formats**  
✔ **Comprehensive error handling**  
✔ **High-performance async operations**  

## 🛠 Tech Stack
| Technology | Purpose |
|------------|---------|
| **Rust** 🦀 | Systems programming language |
| **Actix-web** ⚡ | Web framework |
| **Chrono** 📅 | Date and time handling |
| **Serde** 📦 | Serialization/Deserialization |

## 🚀 Getting Started

### Prerequisites
First, install Rust using [Rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For more details and platform-specific instructions, visit the official Rust installation page:
[🔗 Rust Installation Guide](https://www.rust-lang.org/tools/install)

### Installation
1. Clone the repository:
```bash
git clone https://github.com/yourusername/timestamp_microservice.git
cd timestamp_microservice
```

2. Build the project:
```bash
cargo build
```

3. Run the tests:
```bash
cargo test
```

4. Start the server:
```bash
cargo run
```

The server will start on `http://localhost:8080`.

## 📡 API Endpoints

### 1. Get Current Time
Returns the current time in UTC and Unix timestamp format.

```bash
curl http://localhost:8080/api
```

Response:
```json
{
    "unix": 1706400000000,
    "utc": "Sun, 28 Jan 2024 00:00:00 GMT",
    "timezone": "UTC",
    "local_time": "2024-01-28 00:00:00 +0000"
}
```

### 2. Get Time for Specific Date
```bash
curl http://localhost:8080/api/2024-01-28
```

With timezone:
```bash
curl "http://localhost:8080/api/2024-01-28?timezone=America/New_York"
```

### 3. List Available Timezones
```bash
curl http://localhost:8080/api/timezones
```

Response:
```json
[
    "Africa/Abidjan",
    "America/New_York",
    "Asia/Tokyo",
    "Europe/London",
    ...
]
```

### 4. Get Weekday for Date
```bash
curl http://localhost:8080/api/weekday/2024-01-28
```

Response:
```json
{
    "weekday": "Sunday"
}
```

## 🔍 Error Handling

The API uses structured error responses:

```json
{
    "type": "InvalidDate",
    "message": "input contains invalid characters"
}
```

Error types:
- `InvalidDate`: Invalid date format
- `InvalidTimezone`: Unsupported timezone
- `InternalError`: Server-side error

## 🧪 Testing

Run all tests:
```bash
cargo test
```

Run specific test:
```bash
cargo test test_get_weekday
```

Run tests with output:
```bash
cargo test -- --nocapture
```

## 📂 Project Structure
```
/timestamp_microservice
├── src/
│   ├── main.rs           # Application entry point
│   ├── error.rs          # Error handling
│   └── handlers/         # Request handlers
│       ├── mod.rs        # Handler exports
│       ├── time.rs       # Time endpoints
│       ├── weekday.rs    # Weekday endpoint
│       └── timezones.rs  # Timezone endpoint
├── tests/                # Integration tests
├── Cargo.toml           # Dependencies
└── README.md           # Documentation
```

## 🛠 Development Commands

### Cargo Commands
```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run the application
cargo run

# Run tests
cargo test

# Check for compilation errors
cargo check

# Update dependencies
cargo update

# Clean build artifacts
cargo clean
```

### Development Tips
- Use `cargo watch` for auto-reloading:
  ```bash
  cargo install cargo-watch
  cargo watch -x run
  ```
- Format code:
  ```bash
  cargo fmt
  ```
- Check code style:
  ```bash
  cargo clippy
  ```

## 📚 API Documentation

Full API documentation with examples is available in the [API.md](API.md) file.

## 🤝 Contributing
1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## 📝 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 📖 Full Tutorial & Blog Post
📌 **Read the step-by-step guide on my blog:**  
**👉 [Building a Timestamp Microservice with Rust and Actix-web](https://dev.to/yourusername/building-a-timestamp-microservice-with-rust-and-actix-web)**

The tutorial covers:
- Setting up a Rust project with Actix-web
- Implementing timezone conversions with chrono
- Error handling best practices
- Writing integration tests
- API documentation
- Performance optimization tips

## 🔗 Contact
- GitHub: [@yourusername](https://github.com/yourusername)
- Email: your.email@example.com
- Blog: [dev.to/yourusername](https://dev.to/yourusername)