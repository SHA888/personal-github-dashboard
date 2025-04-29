# Personal GitHub Dashboard

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![Backend CI](https://github.com/SHA888/personal-github-dashboard/actions/workflows/backend.yml/badge.svg)
![Frontend CI](https://github.com/SHA888/personal-github-dashboard/actions/workflows/frontend.yml/badge.svg)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/1d503988e75e42b99abe292ae36f4ce9)](https://app.codacy.com/gh/SHA888/personal-github-dashboard/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![codecov](https://codecov.io/gh/SHA888/personal-github-dashboard/branch/main/graph/badge.svg?token=TOKEN)](https://codecov.io/gh/SHA888/personal-github-dashboard)
![GitHub Copilot enabled](https://img.shields.io/badge/Copilot-Enabled-10cfc9?logo=github)
![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/SHA888/personal-github-dashboard?utm_source=oss&utm_medium=github&utm_campaign=SHA888%2Fpersonal-github-dashboard&labelColor=171717&color=FF570A&link=https%3A%2F%2Fcoderabbit.ai&label=CodeRabbit+Reviews)

</div>

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
- Rust 1.75+
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
# Copy example environment files
cp .env.example .env
```

4. Install dependencies:

```bash
# Install project dependencies
npm install
```

5. Start the development servers:

```bash
# Start both frontend and backend
npm run dev
```

The application will be available at:

- Frontend: http://localhost:3001
- Backend API: http://localhost:3000

## Project Structure

```
personal-github-dashboard/
├── backend/        # Rust backend (Actix Web)
│   ├── src/       # Source code
│   ├── migrations/# Database migrations
│   └── .env       # Environment configuration
├── frontend/      # TypeScript/React frontend
├── docs/         # Comprehensive documentation
└── README.md
```

## Available Scripts

- `npm run dev` - Start both frontend and backend in development mode
- `npm run build` - Build both frontend and backend for production
- `npm run test` - Run tests for both frontend and backend
- `npm run lint` - Run linting for frontend
- `npm run format` - Format all code with Prettier

## API Endpoints

### Health Check

- `GET /api/health` - Check service health status

### Analytics

- `GET /api/analytics/repository/{owner}/{repo}/activity` - Get repository activity data
- `GET /api/analytics/repository/{owner}/{repo}/trends` - Get repository trends

### Data Synchronization

- `POST /api/sync/repository/{owner}/{repo}` - Manually trigger repository data sync

## Documentation

For comprehensive documentation, including setup instructions, architecture details, API reference, and deployment guide, please visit our [Documentation](./docs/README.md).

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please read our [Development Guide](./docs/development/README.md) for detailed contribution guidelines.

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
