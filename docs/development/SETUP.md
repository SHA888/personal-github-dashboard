# Development Setup Guide

## Prerequisites

### Common Requirements
- Node.js 18+ and npm
- Rust 1.75+ and Cargo
- Git

### Web Mode Requirements
- PostgreSQL 15+
- Redis 7+
- Docker (optional, for containerization)

### Desktop Mode Requirements
- Tauri development dependencies:
  - Windows: Microsoft Visual Studio C++ Build Tools
  - macOS: Xcode Command Line Tools
  - Linux: `build-essential`, `libwebkit2gtk-4.0-dev`, other Tauri dependencies
- Logseq (for testing integration)

## Initial Setup

1. **Clone the Repository**
   ```bash
   git clone https://github.com/SHA888/personal-github-dashboard.git
   cd personal-github-dashboard
   ```

2. **Install Frontend Dependencies**
   ```bash
   npm install
   ```

3. **Install Rust Dependencies**
   ```bash
   # For web backend
   cd backend
   cargo build

   # For desktop (Tauri)
   cd src-tauri
   cargo build
   ```

## Web Mode Setup

1. **Database Setup**
   ```bash
   # Start PostgreSQL and Redis (using Docker)
   docker-compose up -d

   # Or configure local instances:
   # PostgreSQL connection string: postgresql://user:password@localhost:5432/dashboard
   # Redis connection string: redis://localhost:6379
   ```

2. **Environment Configuration**
   ```bash
   # Create .env file in backend directory
   cp backend/.env.example backend/.env

   # Configure variables:
   DATABASE_URL=postgresql://user:password@localhost:5432/dashboard
   REDIS_URL=redis://localhost:6379
   GITHUB_CLIENT_ID=your_client_id
   GITHUB_CLIENT_SECRET=your_client_secret
   ```

3. **Database Migrations**
   ```bash
   cd backend
   cargo run --bin migrate
   ```

4. **Start Development Servers**
   ```bash
   # Terminal 1: Frontend
   npm run dev

   # Terminal 2: Backend
   cd backend
   cargo run
   ```

## Desktop Mode Setup

1. **Tauri Setup**
   ```bash
   # Install Tauri CLI
   cargo install tauri-cli

   # Create src-tauri directory if not exists
   cargo tauri init
   ```

2. **Configure Tauri**
   ```bash
   # Edit src-tauri/tauri.conf.json
   # Configure allowed APIs, windows, etc.
   ```

3. **Local Storage Setup**
   ```bash
   # Tauri will handle this automatically
   # Configuration in src-tauri/src/main.rs
   ```

4. **Logseq Integration**
   ```bash
   # Create Logseq graph directory
   mkdir -p ~/personal-github-dashboard-logseq
   ```

5. **Start Desktop Development**
   ```bash
   # Development mode
   npm run tauri dev

   # Build desktop application
   npm run tauri build
   ```

## Development Workflow

### Web Mode Development
1. Make changes to frontend code in `src/`
2. Make changes to backend code in `backend/`
3. Run tests: `npm test` and `cargo test`
4. Check formatting: `npm run format` and `cargo fmt`
5. Check linting: `npm run lint` and `cargo clippy`

### Desktop Mode Development
1. Make changes to frontend code in `src/`
2. Make changes to Tauri backend in `src-tauri/`
3. Test desktop features: `npm run tauri dev`
4. Build for platforms: `npm run tauri build`

### Shared Code Development
1. Keep frontend components mode-agnostic
2. Use environment detection for mode-specific features
3. Maintain consistent API interfaces
4. Test in both web and desktop modes

## Environment Variables

### Web Mode
```env
# Backend (.env)
DATABASE_URL=postgresql://user:password@localhost:5432/dashboard
REDIS_URL=redis://localhost:6379
GITHUB_CLIENT_ID=your_client_id
GITHUB_CLIENT_SECRET=your_client_secret
JWT_SECRET=your_jwt_secret

# Frontend (.env)
VITE_API_URL=http://localhost:8000
VITE_MODE=web
```

### Desktop Mode
```env
# Frontend (.env)
VITE_MODE=desktop

# Tauri will handle secure storage
```

## Testing

### Run All Tests
```bash
# Frontend tests
npm test

# Backend tests
cd backend && cargo test

# Tauri tests
cd src-tauri && cargo test
```

### Run Specific Tests
```bash
# Frontend component tests
npm test -- --watch

# Backend integration tests
cd backend && cargo test --test integration

# Tauri plugin tests
cd src-tauri && cargo test --test plugins
```

## Common Issues

### Web Mode
1. **Database Connection Issues**
   - Check PostgreSQL service is running
   - Verify connection string
   - Check network access

2. **Redis Connection Issues**
   - Check Redis service is running
   - Verify connection string
   - Check memory usage

### Desktop Mode
1. **Tauri Build Issues**
   - Check system dependencies
   - Update Rust and Node.js
   - Clear cache: `npm run tauri clean`

2. **Logseq Integration Issues**
   - Check file permissions
   - Verify graph directory exists
   - Check Logseq installation

## Additional Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Actix-web Documentation](https://actix.rs/docs/)
- [React Documentation](https://react.dev/)
- [Logseq Documentation](https://docs.logseq.com/)
