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

## Environment Variables

Copy `.env.example` to `.env` and fill in the required values. Below are the variables you need to configure:

```ini
# Database Configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/personal_github_dashboard_dev
PG_POOL_MAX=10

# Redis Configuration
REDIS_URL=redis://localhost:6379

# Server Configuration
PORT=8081

# GitHub OAuth Configuration
GITHUB_CLIENT_ID=your_github_client_id_here
GITHUB_CLIENT_SECRET=your_github_client_secret_here
GITHUB_CALLBACK_URL=http://localhost:3000/api/auth/callback
GITHUB_OAUTH_SCOPES=read:org,read:user,repo

# JWT Configuration
JWT_SECRET=your_secure_jwt_secret_here
JWT_EXPIRES_IN=24h
```

**Note:**

- Never commit your `.env` file or any credentials to version control.
- Only use `.env.example` as a template.
- All credentials and secrets should be managed through environment variables.

---

For more details on each variable, see the comments in `.env.example`.

## Setup

1. Install dependencies:

   ```bash
   cargo build
   ```

2. Set up the database:

   ```bash
   cargo run --bin migrate
   ```

3. Start the server:
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

## Integration Tests

The backend includes comprehensive integration tests for caching logic:

- `tests/user_cache_integration.rs` – User cache integration tests
- `tests/repository_cache_integration.rs` – Repository cache integration tests
- `tests/organization_cache_integration.rs` – Organization cache integration tests
- `tests/activity_cache_integration.rs` – Activity cache integration tests

To run all backend integration tests:

```bash
cargo test --test user_cache_integration
cargo test --test repository_cache_integration
cargo test --test organization_cache_integration
cargo test --test activity_cache_integration
```

Or simply:

```bash
cargo test
```

## API Documentation

For detailed API documentation, please refer to the [API Documentation](../../docs/api/README.md).

## Contributing

Please read our [Development Guide](../../docs/development/README.md) for details on our code of conduct and the process for submitting pull requests.

## License

MIT
