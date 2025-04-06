# Development Guide

This guide provides information for developers contributing to the GitHub Dashboard project.

## Development Environment Setup

### Prerequisites
- Rust (latest stable)
- Node.js (v16+)
- PostgreSQL (v12+)
- Git

### Recommended Tools
- VS Code with extensions:
  - Rust Analyzer
  - ESLint
  - Prettier
  - SQLx

## Development Workflow

### 1. Fork and Clone
```bash
git clone https://github.com/your-username/github-dashboard.git
cd github-dashboard
```

### 2. Create Feature Branch
```bash
git checkout -b feature/your-feature-name
```

### 3. Set Up Development Environment
```bash
# Backend
cd backend
cp .env.example .env
cargo build

# Frontend
cd ../frontend
cp .env.example .env
npm install
```

### 4. Make Changes
- Follow coding standards
- Write tests
- Update documentation

### 5. Run Tests
```bash
# Backend tests
cd backend
cargo test

# Frontend tests
cd ../frontend
npm test
```

### 6. Submit Pull Request
- Create PR from feature branch
- Include description of changes
- Reference related issues

## Coding Standards

### Rust (Backend)
- Follow Rust style guide
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Document public APIs

### TypeScript/React (Frontend)
- Use TypeScript strict mode
- Follow React best practices
- Use functional components
- Implement proper error handling

## Database Migrations

### Creating Migrations
```bash
cd backend
sqlx migrate add your_migration_name
```

### Running Migrations
```bash
sqlx migrate run
```

### Rolling Back
```bash
sqlx migrate revert
```

## Testing

### Backend Tests
- Unit tests in `src/`
- Integration tests in `tests/`
- Run with `cargo test`

### Frontend Tests
- Component tests with React Testing Library
- Integration tests with Cypress
- Run with `npm test`

## Debugging

### Backend
- Use `RUST_LOG=debug` for detailed logs
- Attach debugger with VS Code
- Use `cargo run` for development

### Frontend
- Use React DevTools
- Enable source maps
- Use browser dev tools

## Performance Optimization

### Backend
- Profile with `cargo flamegraph`
- Optimize database queries
- Implement caching

### Frontend
- Use React.memo
- Implement code splitting
- Optimize bundle size

## Security Considerations

- Validate all inputs
- Sanitize database queries
- Handle secrets properly
- Implement rate limiting

## Documentation

- Update API docs
- Add code comments
- Update README files
- Document breaking changes 