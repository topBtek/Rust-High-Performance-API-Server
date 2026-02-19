# Rust High-Performance API Server

A production-ready, high-performance RESTful API server built with Rust and Actix-web, demonstrating best-in-class practices for performance, safety, maintainability, and scalability.

## 🚀 Features

- **High Performance**: Built on Actix-web's actor model for maximum throughput and low latency
- **Async-First Design**: Fully asynchronous using Tokio runtime
- **Structured Logging**: Request logging with trace IDs using `tracing`
- **Error Handling**: Comprehensive error handling with `thiserror` and `anyhow`
- **Configuration**: Environment-based configuration with `.env` support
- **Middleware**: Request logging and API key authentication
- **CORS Support**: Configurable CORS middleware
- **Health Checks**: Built-in health check endpoint
- **CRUD Operations**: Example task management endpoints
- **Testing**: Unit and integration tests
- **Docker Support**: Production-ready Dockerfile and docker-compose.yml
- **Benchmarking**: Built-in benchmarks for performance testing

## 🏗️ Architecture

The project follows a clean, modular architecture:

```
src/
├── main.rs           # Application entry point
├── lib.rs            # Application setup and configuration
├── config.rs         # Configuration management
├── errors.rs         # Error types and handling
├── state.rs          # Shared application state
├── models.rs         # Data models and DTOs
├── handlers.rs       # Request handlers
├── routes.rs         # Route configuration
└── middleware/
    ├── mod.rs
    ├── logging.rs    # Request logging middleware
    └── auth.rs       # API key authentication middleware
```

## 📦 Tech Stack

### Core Dependencies

- **actix-web** (4.8): High-performance web framework
- **tokio** (1.40): Async runtime
- **serde** + **serde_json**: JSON serialization/deserialization
- **thiserror** + **anyhow**: Error handling
- **tracing** + **tracing-subscriber**: Structured logging
- **dashmap**: Thread-safe concurrent hash map
- **uuid**: UUID generation
- **chrono**: Date/time handling
- **config** + **dotenvy**: Configuration management
- **actix-cors**: CORS middleware

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ (stable)
- Cargo (comes with Rust)

### Installation

1. **Clone the repository** (or navigate to the project directory):
   ```bash
   cd Rust-High-Performance-API-Server-1
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the server**:
   ```bash
   cargo run
   ```

   Or with custom configuration:
   ```bash
   SERVER__PORT=3000 API__API_KEY=my-secret-key cargo run
   ```

### Using Docker

1. **Build and run with Docker Compose**:
   ```bash
   docker-compose up --build
   ```

2. **Or build the Docker image manually**:
   ```bash
   docker build -t rust-api-server .
   docker run -p 8080:8080 -e API__API_KEY=my-secret-key rust-api-server
   ```

## ⚙️ Configuration

Configuration is managed through environment variables. Create a `.env` file in the project root:

```env
# Server Configuration
SERVER__ADDRESS=0.0.0.0
SERVER__PORT=8080
SERVER__WORKERS=4

# API Configuration
API__API_KEY=dev-api-key-change-in-production

# Logging
RUST_LOG=info
```

### Configuration Options

- `SERVER__ADDRESS`: Bind address (default: `0.0.0.0`)
- `SERVER__PORT`: Port number (default: `8080`)
- `SERVER__WORKERS`: Number of worker threads (default: number of CPU cores)
- `API__API_KEY`: API key for authentication (default: `dev-api-key-change-in-production`)
- `RUST_LOG`: Logging level (default: `info`)

## 📡 API Endpoints

### Health Check

**GET** `/health`

Public endpoint to check server health.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2026-01-15T10:30:00Z",
  "version": "0.1.0"
}
```

### Tasks API

All task endpoints require the `X-API-Key` header with a valid API key.

#### Get All Tasks

**GET** `/api/v1/tasks`

**Headers:**
```
X-API-Key: your-api-key
```

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Example Task",
    "description": "Task description",
    "completed": false,
    "created_at": "2026-01-15T10:30:00Z",
    "updated_at": "2026-01-15T10:30:00Z"
  }
]
```

#### Get Task by ID

**GET** `/api/v1/tasks/{id}`

**Headers:**
```
X-API-Key: your-api-key
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Example Task",
  "description": "Task description",
  "completed": false,
  "created_at": "2026-01-15T10:30:00Z",
  "updated_at": "2026-01-15T10:30:00Z"
}
```

#### Create Task

**POST** `/api/v1/tasks`

**Headers:**
```
X-API-Key: your-api-key
Content-Type: application/json
```

**Request Body:**
```json
{
  "title": "New Task",
  "description": "Optional description"
}
```

**Response:** `201 Created`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "New Task",
  "description": "Optional description",
  "completed": false,
  "created_at": "2026-01-15T10:30:00Z",
  "updated_at": "2026-01-15T10:30:00Z"
}
```

#### Update Task

**PUT** `/api/v1/tasks/{id}`

**Headers:**
```
X-API-Key: your-api-key
Content-Type: application/json
```

**Request Body:**
```json
{
  "title": "Updated Task",
  "description": "Updated description",
  "completed": true
}
```

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Updated Task",
  "description": "Updated description",
  "completed": true,
  "created_at": "2026-01-15T10:30:00Z",
  "updated_at": "2026-01-15T10:35:00Z"
}
```

#### Delete Task

**DELETE** `/api/v1/tasks/{id}`

**Headers:**
```
X-API-Key: your-api-key
```

**Response:** `204 No Content`

### Error Responses

All endpoints return standardized error responses:

```json
{
  "error": "Error Type",
  "message": "Error message"
}
```

**Status Codes:**
- `400 Bad Request`: Validation errors
- `401 Unauthorized`: Missing or invalid API key
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server errors

## 🐳 Docker Deployment

### Build Image

```bash
docker build -t rust-api-server .
```

### Run Container

```bash
docker run -d \
  -p 8080:8080 \
  -e API__API_KEY=production-api-key \
  -e RUST_LOG=info \
  --name rust-api-server \
  rust-api-server
```

### Docker Compose

```bash
docker-compose up -d
```

## 📞 Support

- Telegram: https://t.me/topBtek
- Twitter: https://x.com/topBtek
