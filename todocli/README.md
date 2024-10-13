# Todo API

Todo API is a simple and powerful RESTful API for managing todo items, built with Rust and Axum.

## Features

- Create new todo items
- List all todo items
- Get a specific todo item
- Update existing todo items
- Delete todo items

## Installation

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository:

   ```
   git clone https://github.com/yourusername/todocli.git
   cd todocli
   ```

3. Build the project:

   ```
   cargo build --release
   ```

## Running the Server

To start the server, run:

```rust
cargo run
```

The server will start on `http://localhost:3000`.

## API Usage

Here are some curl commands to interact with the API:

### Create a new todo

```bash
curl -X POST http://localhost:3000/todos \
-H "Content-Type: application/json" \
-d '{"title": "Learn Rust", "description": "Study Rust programming language"}'
```

### List all todos

```bash
curl http://localhost:3000/todos
```

### Get a specific todo

```bash
curl http://localhost:3000/todos/1
```

### Update a todo

```bash
curl -X PUT http://localhost:3000/todos/1 -H "Content-Type: application/json" -d '{"title": "Learn Rust", "description": "Study Rust programming language"}'
```

### Delete a todo

```bash
curl -X DELETE http://localhost:3000/todos/1
```
