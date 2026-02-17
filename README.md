# 🦀 Rusty User API

A high-performance, thread-safe REST API built with **Rust**, **Axum**, and **Tokio**.
This project demonstrates how to handle shared state concurrency safely using `Arc<Mutex<T>>` in an asynchronous environment.

## 🚀 Tech Stack
- **Framework:** [Axum](https://github.com/tokio-rs/axum) (Ergonomic and modular web framework)
- **Runtime:** [Tokio](https://tokio.rs/) (Asynchronous I/O)
- **Serialization:** Serde & Serde JSON
- **Concurrency:** `std::sync::Mutex` for thread-safe state management.

## ⚡ Features
- **In-Memory Storage:** Fast data access with zero external dependencies.
- **Thread Safety:** Uses Atomic Reference Counting (`Arc`) and Mutexes to prevent data races.
- **RESTful Endpoints:**
    - `POST /users`: Create a new user (Auto-incrementing IDs).
    - `GET /users`: Retrieve all users.
    - `GET /users/:id`: Retrieve a specific user.
    - `DELETE /users/:id`: Remove a user.

## 🛠️ How to Run
```bash
cargo run
# The server will start on http://localhost:3000
