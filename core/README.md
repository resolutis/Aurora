# Aurora3D Core - Rust Axum API Boilerplate

A comprehensive Rust web API boilerplate built with [Axum](https://github.com/tokio-rs/axum), featuring modern async/await patterns, middleware, error handling, and a clean project structure.

## 🚀 Features

- **Fast & Modern**: Built on Axum with Tokio async runtime
- **RESTful API**: Complete CRUD operations with proper HTTP methods
- **Error Handling**: Custom error types with proper HTTP status codes
- **Middleware**: CORS, logging, and request tracing
- **Validation**: Input validation with meaningful error messages
- **Logging**: Structured logging with tracing
- **Type Safety**: Full Rust type safety with serde serialization

## 📋 Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- Cargo (comes with Rust)

## 🛠️ Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd core
```

2. Install dependencies:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://0.0.0.0:3000`

## 📚 API Documentation

### Base URL
```
http://localhost:3000
```

### Endpoints

#### Health Check
Check if the server is running.

**GET** `/health`

**Response:**
```
OK
```

**Example:**
```bash
curl http://localhost:3000/health
```

---

#### Users API

##### Get All Users
Retrieve a list of users with optional pagination.

**GET** `/api/users`

**Query Parameters:**
- `limit` (optional): Number of users to return (default: 10)
- `offset` (optional): Number of users to skip (default: 0)

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john@example.com"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "Jane Smith",
    "email": "jane@example.com"
  }
]
```

**Examples:**
```bash
# Get all users
curl http://localhost:3000/api/users

# Get users with pagination
curl "http://localhost:3000/api/users?limit=5&offset=10"
```

---

##### Get User by ID
Retrieve a specific user by their UUID.

**GET** `/api/users/{id}`

**Path Parameters:**
- `id`: User UUID

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "John Doe",
  "email": "john@example.com"
}
```

**Example:**
```bash
curl http://localhost:3000/api/users/550e8400-e29b-41d4-a716-446655440000
```

---

##### Create User
Create a new user.

**POST** `/api/users`

**Request Body:**
```json
{
  "name": "John Doe",
  "email": "john@example.com"
}
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "John Doe",
  "email": "john@example.com"
}
```

**Example:**
```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john@example.com"
  }'
```

---

##### Update User
Update an existing user. All fields are optional.

**PUT** `/api/users/{id}`

**Path Parameters:**
- `id`: User UUID

**Request Body:**
```json
{
  "name": "John Updated",
  "email": "john.updated@example.com"
}
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "John Updated",
  "email": "john.updated@example.com"
}
```

**Example:**
```bash
curl -X PUT http://localhost:3000/api/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Updated",
    "email": "john.updated@example.com"
  }'
```

---

##### Delete User
Delete a user by their UUID.

**DELETE** `/api/users/{id}`

**Path Parameters:**
- `id`: User UUID

**Response:**
- Status: `204 No Content`

**Example:**
```bash
curl -X DELETE http://localhost:3000/api/users/550e8400-e29b-41d4-a716-446655440000
```

## 🔧 Error Handling

The API returns structured error responses with appropriate HTTP status codes:

### Error Response Format
```json
{
  "error": "Error message description"
}
```

### Common Error Codes

- **400 Bad Request**: Validation errors (e.g., invalid email format, empty name)
- **404 Not Found**: Resource not found
- **500 Internal Server Error**: Server-side errors

### Example Error Responses

**Validation Error:**
```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "", "email": "invalid-email"}'
```

**Response:**
```json
{
  "error": "Name cannot be empty"
}
```

**Not Found Error:**
```bash
curl http://localhost:3000/api/users/00000000-0000-0000-0000-000000000000
```

**Response:**
```json
{
  "error": "Resource not found"
}
```

## 🏗️ Project Structure

```
core/
├── Cargo.toml          # Dependencies and project configuration
├── src/
│   └── main.rs         # Main application code
└── README.md           # This file
```

## 🔧 Development

### Running in Development Mode
```bash
cargo run
```

### Building for Production
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## 📦 Dependencies

- **axum**: Modern web framework for Rust
- **tokio**: Async runtime
- **tower**: Middleware and service abstractions
- **tower-http**: HTTP-specific middleware (CORS, tracing)
- **serde**: Serialization/deserialization
- **tracing**: Structured logging
- **uuid**: UUID generation and parsing

## 🚀 Next Steps

This boilerplate provides a solid foundation for building production-ready APIs. Consider adding:

- **Database Integration**: Add SQLx, Diesel, or another ORM
- **Authentication**: JWT tokens, OAuth, or session-based auth
- **Rate Limiting**: Protect your API from abuse
- **API Documentation**: OpenAPI/Swagger documentation
- **Testing**: Unit and integration tests
- **Docker**: Containerization for deployment
- **Environment Configuration**: Config management for different environments

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📞 Support

If you have any questions or need help, please open an issue on GitHub.
