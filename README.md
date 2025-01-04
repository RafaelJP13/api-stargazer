# Actix Web Hello World

This project demonstrates a simple HTTP server using the [Actix Web](https://actix.rs/) framework in Rust. The server responds with a "Hello World!" message when accessed at the root URL (`/`).

## Requirements

- **Rust**: Latest stable version recommended.
- **Cargo**: Comes bundled with Rust.

## Installation

1. **Clone the Repository**

    ```bash
    git clone <repository-url>
    cd <repository-folder>
    ```

2. **Add Actix Web Dependency**

    Ensure your `Cargo.toml` includes:

    ```toml
    [dependencies]
    actix-web = "4"
    ```

3. **Build the Project**

    ```bash
    cargo build
    ```

## Usage

1. **Run the Server**

    ```bash
    cargo run
    ```

2. **Access the Server**

    Open your browser or use a tool like `curl` to navigate to:

    ```
    http://127.0.0.1:8080
    ```

Sets up the Actix Web runtime and starts the server.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

For more information about Actix Web, visit the [official documentation](https://actix.rs/).

