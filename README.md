# Personal GitHub Dashboard

An open-source dashboard to track and analyze your GitHub activity across repositories and organizations.

## Overview

This project consists of:
- **Backend**: Built with Rust using Actix Web, fetching data from the GitHub API and storing in PostgreSQL.
- **Frontend**: A TypeScript/React application with Recharts for visualizations.
- **Cache**: Redis for real-time data and rate limiting.

## Features
- View and analyze your GitHub repositories and organizations
- Track commit activity, issues, and pull requests
- Real-time updates via WebSocket
- Advanced analytics and custom reports
- Modern, responsive web interface

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
     GITHUB_TOKEN=your_personal_access_token
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

### Analytics
- `GET /analytics/repository/{owner}/{repo}/activity` - Get repository activity data
- `GET /analytics/repository/{owner}/{repo}/trends` - Get repository trends

### Data Synchronization
- `POST /sync/repository/{owner}/{repo}` - Manually trigger repository data sync
