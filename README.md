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
