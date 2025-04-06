# GitHub Dashboard

A personal GitHub projects management and analytics dashboard to track activity, manage tasks, and analyze trends across repositories.

## Overview

This project consists of:
- **Backend**: Built with Rust using Actix Web, fetching data from the GitHub API and storing tasks in SQLite.
- **Frontend**: A TypeScript/React application with Chart.js for visualizations.

## Features
- View a list of your GitHub repositories.
- Display commit activity trends in a chart.
- (Planned) Manage and prioritize tasks tied to repositories.
- Automatic data synchronization with GitHub
- RESTful API endpoints for analytics and data sync
- Modern web interface

## Documentation

For comprehensive documentation, including setup instructions, architecture details, API reference, and deployment guide, please visit our [Documentation](./docs/README.md).

## Prerequisites
- Rust (install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rust-lang.org | sh`)
- Node.js and npm (for the frontend)
- A GitHub Personal Access Token (PAT) with `repo` and `user` scopes

## Setup Instructions

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/github-dashboard.git
   cd github-dashboard
   ```

2. **Backend Setup**:
   ```bash
   cd backend
   cargo build
   ```
   - Create a `.env` file in `backend/` with:
     ```
     GITHUB_TOKEN=your_personal_access_token
     DATABASE_URL=database.db
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
github-dashboard/
├── backend/        # Rust backend (Actix Web)
├── frontend/       # TypeScript/React frontend
├── docs/          # Comprehensive documentation
├── .gitignore
└── README.md
```

## Future Plans
- Add task management with CRUD operations.
- Deploy to a VPS with Nginx.
- Enhance analytics with more GitHub API data (issues, PRs, etc.).

## Contributing
Feel free to fork, submit PRs, or open issues!

## License
MIT

## API Endpoints

### Analytics
- `GET /analytics/repository/{owner}/{repo}/activity` - Get repository activity data
- `GET /analytics/repository/{owner}/{repo}/trends` - Get repository trends

### Data Synchronization
- `POST /sync/repository/{owner}/{repo}` - Manually trigger repository data sync

## Development

The application consists of:

- Backend (Rust + Actix-web)
  - GitHub API integration using octocrab
  - PostgreSQL database for data storage
  - Automatic hourly data synchronization
  - Manual sync triggers via API

- Frontend (React + TypeScript)
  - Modern UI components
  - Real-time data visualization
  - Responsive design

## Architecture

The system uses:
- Rust for backend services
- PostgreSQL for data storage
- GitHub API for repository data
- React for frontend interface

Data flow:
1. GitHub data is fetched via API
2. Stored in PostgreSQL database
3. Processed for analytics
4. Served via REST API
5. Visualized in web interface
