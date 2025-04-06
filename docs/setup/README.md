# Setup Guide

This guide provides detailed instructions for setting up the GitHub Dashboard project.

## Prerequisites

### System Requirements
- Rust (latest stable version)
- Node.js (v16 or later)
- npm (v7 or later)
- PostgreSQL (v12 or later)
- Git

### Required Accounts
- GitHub account with a Personal Access Token (PAT)
  - Required scopes: `repo`, `user`

## Installation Steps

### 1. Clone the Repository
```bash
git clone https://github.com/SHA888/github-dashboard.git
cd github-dashboard
```

### 2. Backend Setup

#### Environment Configuration
Create a `.env` file in the `backend` directory:
```bash
cd backend
touch .env
```

Add the following environment variables:
```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/github_dashboard
GITHUB_TOKEN=your_github_personal_access_token
PORT=8080
```

#### Database Setup
1. Create a PostgreSQL database:
```bash
createdb github_dashboard
```

2. Run database migrations:
```bash
sqlx database create
sqlx migrate run
```

#### Build and Run
```bash
cargo build
cargo run
```

### 3. Frontend Setup

#### Install Dependencies
```bash
cd frontend
npm install
```

#### Environment Configuration
Create a `.env` file in the `frontend` directory:
```bash
touch .env
```

Add the following environment variables:
```env
VITE_API_URL=http://localhost:8080
```

#### Run Development Server
```bash
npm run dev
```

## Accessing the Application

- Backend API: `http://localhost:8080`
- Frontend Application: `http://localhost:3001`

## Troubleshooting

### Common Issues

1. **Database Connection Issues**
   - Ensure PostgreSQL is running
   - Verify database credentials in `.env`
   - Check if the database exists

2. **GitHub API Rate Limits**
   - Verify your GitHub token has the correct scopes
   - Check your token's rate limit status

3. **Port Conflicts**
   - Ensure no other services are using ports 8080 or 3001
   - Update port numbers in `.env` files if needed

### Getting Help

If you encounter issues not covered in this guide:
1. Check the [Development Guide](../development/README.md)
2. Review the [Architecture Documentation](../architecture/README.md)
3. Open an issue on GitHub 