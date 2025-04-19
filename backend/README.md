# Personal GitHub Dashboard Backend

A high-performance backend service built with Rust and Actix Web for the Personal GitHub Dashboard.

## Overview

The backend is built using:

- Rust with Actix Web framework
- PostgreSQL for data storage
- Redis for caching and real-time features
- GitHub API integration via octocrab
- WebSocket support for real-time updates

## Prerequisites

- Rust (latest stable version)
- PostgreSQL (v14 or later)
- Redis (v6 or later)
- GitHub Personal Access Token (PAT) with required scopes

## Setup

1. Install dependencies:

   ```bash
   cargo build
   ```

2. Create a `.env` file in the backend directory:

   ```
   GITHUB_PERSONAL_ACCESS_TOKEN=your_personal_access_token
   DATABASE_URL=postgresql://user:password@localhost:5432/personal_github_dashboard
   REDIS_URL=redis://localhost:6379
   PORT=8080
   ```

3. Set up the database:

   ```bash
   cargo run --bin migrate
   ```

4. Start the server:
   ```bash
   cargo run
   ```

## Development

### Project Structure

```
backend/
├── src/
│   ├── api/          # API route handlers
│   ├── config/       # Configuration management
│   ├── db/           # Database models and migrations
│   ├── github/       # GitHub API integration
│   ├── services/     # Business logic
│   ├── utils/        # Utility functions
│   ├── websocket/    # WebSocket handlers
│   └── main.rs       # Application entry point
├── migrations/       # Database migrations
└── Cargo.toml        # Rust dependencies
```

### Key Features

- RESTful API endpoints
- Real-time WebSocket updates
- Automatic data synchronization
- Rate limiting and caching
- Secure authentication
- Efficient database queries

### Available Commands

- `cargo run` - Start the development server
- `cargo build --release` - Build for production
- `cargo test` - Run tests
- `cargo clippy` - Run linter
- `cargo fmt` - Format code

## API Documentation

For detailed API documentation, please refer to the [API Documentation](../../docs/api/README.md).

## Contributing

Please read our [Development Guide](../../docs/development/README.md) for details on our code of conduct and the process for submitting pull requests.

## License

MIT
