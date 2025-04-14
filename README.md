# Personal GitHub Dashboard

A personalized dashboard that provides insights and analytics for GitHub repositories and activities.

## Features

- Repository overview with key metrics
- Activity analytics and visualizations
- Notification center
- Performance metrics dashboard
- Customizable layouts with dark/light theme

## Tech Stack

### Frontend
- Vite + React
- TypeScript
- Tailwind CSS
- React Query
- Vitest for testing

### Backend
- Rust with Actix-web
- PostgreSQL with SQLx
- Redis for caching
- GitHub API integration
- WebSocket support

## Development Setup

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Docker and Docker Compose
- GitHub API token (for development)

### Getting Started

1. Clone the repository:
```bash
git clone https://github.com/yourusername/personal-github-dashboard.git
cd personal-github-dashboard
```

2. Start the development databases:
```bash
docker-compose up -d
```

3. Set up environment variables:
```bash
# Frontend
cp frontend/.env.example frontend/.env
# Backend
cp backend/.env.example backend/.env
```

4. Install frontend dependencies:
```bash
cd frontend
npm install
cd ..
```

5. Install Rust dependencies and run migrations:
```bash
cd backend
cargo install sqlx-cli
sqlx database create
sqlx migrate run
cd ..
```

6. Start the development servers:
```bash
# Start both frontend and backend
npm run dev

# Or individually:
npm run dev:frontend
npm run dev:backend
```

The application will be available at:
- Frontend: http://localhost:3001
- Backend API: http://localhost:3000

## Available Scripts

- `npm run dev` - Start both frontend and backend in development mode
- `npm run build` - Build both frontend and backend for production
- `npm run test` - Run tests for both frontend and backend
- `npm run lint` - Run linting for frontend
- `npm run format` - Format all code with Prettier

## Database Migrations

```bash
cd backend

# Create a new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Overview

This project consists of:
- **Backend**: Built with Rust using Actix Web, fetching data from the GitHub API and storing in PostgreSQL.
- **Frontend**: A TypeScript/React application with Recharts for visualizations.
- **Cache**: Redis for real-time data and rate limiting.

## Documentation

For comprehensive documentation, including setup instructions, architecture details, API reference, and deployment guide, please visit our [Documentation](./docs/README.md).

## Prerequisites
- Rust (install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rust-lang.org | sh`)
- Node.js and npm (for the frontend)
- PostgreSQL (for the database)
- Redis (for caching)
- A GitHub Personal Access Token (PAT) with `repo` and `user` scopes

## Setup Instructions

1. **Clone the repository**:
   ```bash
   git clone https://github.com/SHA888/personal-github-dashboard.git
   cd personal-github-dashboard
   ```

2. **Backend Setup**:
   ```bash
   cd backend
   cargo build
   ```
   - Create a `.env` file in `backend/` with:
     ```
     GITHUB_PERSONAL_ACCESS_TOKEN=your_personal_access_token
     DATABASE_URL=postgresql://user:password@localhost:5432/personal_github_dashboard
     REDIS_URL=redis://localhost:6379
     PORT=8080
     ```
   - Run: `cargo run`

3. **Frontend Setup**:
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

4. **Access**:
   - Backend: `http://localhost:8080`
   - Frontend: `http://localhost:5173`

## Project Structure
```
personal-github-dashboard/
├── backend/        # Rust backend (Actix Web)
│   ├── src/       # Source code
│   ├── migrations/# Database migrations
│   └── .env       # Environment configuration
├── frontend/       # TypeScript/React frontend
├── docs/          # Comprehensive documentation
├── .gitignore
└── README.md
```

## Development

The application consists of:

- Backend (Rust + Actix-web)
  - GitHub API integration using octocrab
  - PostgreSQL database for data storage
  - Redis for caching and real-time data
  - WebSocket support for live updates
  - Automatic data synchronization
  - Manual sync triggers via API
  - Health monitoring endpoints

- Frontend (React + TypeScript)
  - Modern UI components with Tailwind CSS
  - Real-time data visualization with Recharts
  - WebSocket integration for live updates
  - Responsive design
  - Type-safe development

## Architecture

The system uses:
- Rust for backend services
- PostgreSQL for data storage
- Redis for caching and real-time features
- GitHub API for repository data
- React + TypeScript for frontend interface

Data flow:
1. GitHub data is fetched via API
2. Stored in PostgreSQL database
3. Cached in Redis for performance
4. Processed for analytics
5. Served via REST API and WebSocket
6. Visualized in web interface

## Contributing
Feel free to fork, submit PRs, or open issues! Please read our [Development Guide](./docs/development/README.md) for details on how to contribute.

## License
MIT

## API Endpoints

### Health Check
- `GET /api/health` - Check service health status

### Analytics
- `GET /api/analytics/repository/{owner}/{repo}/activity` - Get repository activity data
- `GET /api/analytics/repository/{owner}/{repo}/trends` - Get repository trends

### Data Synchronization
- `POST /api/sync/repository/{owner}/{repo}` - Manually trigger repository data sync
