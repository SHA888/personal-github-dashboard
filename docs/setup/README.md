# Setup Guide

This guide provides detailed instructions for setting up the GitHub Dashboard development environment.

## Prerequisites

### System Requirements

- Operating System: Windows 10/11, macOS, or Linux
- RAM: 8GB minimum (16GB recommended)
- Storage: 10GB free space
- Internet connection for package downloads

### Required Software

1. **Git**

   - Windows: [Git for Windows](https://git-scm.com/download/win)
   - macOS: `brew install git`
   - Linux: `sudo apt install git`

2. **Rust Toolchain**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup default stable
   rustup component add rustfmt clippy
   ```

3. **Node.js and npm**

   - Windows/macOS: [Node.js Downloads](https://nodejs.org/)
   - Linux:
     ```bash
     curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
     sudo apt install -y nodejs
     ```

4. **PostgreSQL**

   - Windows: [PostgreSQL Downloads](https://www.postgresql.org/download/windows/)
   - macOS: `brew install postgresql`
   - Linux: `sudo apt install postgresql postgresql-contrib`

5. **Docker** (optional, for containerized development)
   - [Docker Desktop](https://www.docker.com/products/docker-desktop)

## Project Setup

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/github-dashboard.git
cd github-dashboard
```

### 2. Backend Setup

#### Install Dependencies

```bash
cd backend
cargo build
```

#### Database Setup

1. Create PostgreSQL database:

   ```bash
   createdb github_dashboard
   ```

2. Run migrations:
   ```bash
   cargo sqlx migrate run
   ```

#### Environment Configuration

Create `.env` file in the backend directory:

```env
GITHUB_PERSONAL_ACCESS_TOKEN=your_github_personal_access_token
DATABASE_URL=postgres://postgres:postgres@localhost:5432/github_dashboard
PORT=8080
```

#### Start Development Server

```bash
cargo run
```

### 3. Frontend Setup

#### Install Dependencies

```bash
cd frontend
npm install
```

#### Environment Configuration

Create `.env` file in the frontend directory:

```env
REACT_APP_API_URL=http://localhost:8080/api/v1
REACT_APP_WS_URL=ws://localhost:8080/api/v1/ws
```

#### Start Development Server

```bash
npm start
```

## Using Docker for Local Development

Alternatively, you can use Docker Compose to set up and run the entire development environment (frontend, backend, database, Redis) with a single command.

1.  **Install Docker and Docker Compose:** Follow the official [Docker documentation](https://docs.docker.com/get-docker/).
2.  **Create a `.env` file:** In the project root directory, create a file named `.env` and add your GitHub Personal Access Token:
    ```env
    GITHUB_PERSONAL_ACCESS_TOKEN=your_github_personal_access_token
    ```
    Ensure this file is added to your `.gitignore`.
3.  **Run Docker Compose:** Open a terminal in the project root directory and run:

    ```bash
    docker-compose up --build
    ```

    This command will build the Docker images for the frontend and backend (if they don't exist or have changed) and start all the services defined in `docker-compose.yml`.

    - The frontend will be accessible at `http://localhost:3000`.
    - The backend API will be accessible at `http://localhost:8080`.
    - PostgreSQL will be running on port `5432`.
    - Redis will be running on port `6379`.

4.  **Running Migrations (First Time):** The backend service in Docker Compose doesn't automatically run database migrations. After starting the containers for the first time, you might need to run migrations manually. Open another terminal:
    ```bash
    docker-compose exec backend sqlx database create
    docker-compose exec backend sqlx migrate run
    ```
5.  **Stopping the Environment:** Press `Ctrl+C` in the terminal where `docker-compose up` is running. To remove the containers and volumes, run:
    ```bash
    docker-compose down -v
    ```

## Development Workflow

### Backend Development

1. **Code Structure**

   ```
   backend/
   ├── src/
   │   ├── main.rs
   │   ├── routes/
   │   ├── models/
   │   ├── services/
   │   └── utils/
   ├── migrations/
   └── tests/
   ```

2. **Running Tests**

   ```bash
   cargo test
   ```

3. **Code Formatting**

   ```bash
   cargo fmt
   ```

4. **Linting**
   ```bash
   cargo clippy
   ```

### Frontend Development

1. **Code Structure**

   ```
   frontend/
   ├── src/
   │   ├── components/
   │   ├── pages/
   │   ├── services/
   │   ├── store/
   │   └── utils/
   ├── public/
   └── tests/
   ```

2. **Running Tests**

   ```bash
   npm test
   ```

3. **Code Formatting**

   ```bash
   npm run format
   ```

4. **Linting**
   ```bash
   npm run lint
   ```

## Database Management

### Schema Updates

1. Create new migration:

   ```bash
   cargo sqlx migrate add <migration_name>
   ```

2. Apply migrations:

   ```bash
   cargo sqlx migrate run
   ```

3. Rollback migration:
   ```bash
   cargo sqlx migrate revert
   ```

### Database Backup

```bash
pg_dump -U postgres github_dashboard > backup.sql
```

### Database Restore

```bash
psql -U postgres github_dashboard < backup.sql
```

## Redis Setup

### Installation

1. **Windows**:

   - Download Redis from [Redis for Windows](https://github.com/microsoftarchive/redis/releases)
   - Run the installer and follow the setup wizard
   - Redis will be installed as a Windows service

2. **macOS**:

   ```bash
   brew install redis
   brew services start redis
   ```

3. **Linux (Ubuntu/Debian)**:
   ```bash
   sudo apt update
   sudo apt install redis-server
   sudo systemctl enable redis-server
   sudo systemctl start redis-server
   ```

### Configuration

1. **Basic Configuration**

   - Default port: 6379
   - Default host: localhost
   - No password by default (development only)

2. **Security Configuration**

   - Set a password in production:

     ```bash
     # Edit Redis configuration
     sudo nano /etc/redis/redis.conf

     # Add or modify these lines:
     requirepass your_secure_password
     bind 127.0.0.1
     ```

   - Restart Redis after configuration changes:
     ```bash
     sudo systemctl restart redis-server
     ```

3. **Environment Configuration**
   Add to your backend `.env` file:
   ```env
   REDIS_URL=redis://localhost:6379
   # If using password:
   REDIS_URL=redis://:your_secure_password@localhost:6379
   ```

### Connection Testing

1. **Basic Connection Test**:

   ```bash
   redis-cli ping
   # Should return: PONG
   ```

2. **With Password**:

   ```bash
   redis-cli -a your_secure_password ping
   ```

3. **From Application**:
   ```bash
   # Test Redis connection from your application
   cargo run --bin test-redis-connection
   ```

### Troubleshooting

1. **Redis Service Not Running**

   ```bash
   # Check Redis status
   sudo systemctl status redis-server

   # Start Redis if stopped
   sudo systemctl start redis-server
   ```

2. **Connection Refused**

   - Verify Redis is running
   - Check firewall settings
   - Ensure correct port (6379)
   - Verify bind address in redis.conf

3. **Authentication Failed**

   - Verify password in redis.conf
   - Check REDIS_URL format in .env
   - Ensure password is properly escaped in URL

4. **Memory Issues**

   - Monitor memory usage:
     ```bash
     redis-cli info memory
     ```
   - Set maxmemory in redis.conf
   - Configure eviction policy

5. **Performance Issues**
   - Monitor Redis performance:
     ```bash
     redis-cli info stats
     ```
   - Check for slow queries:
     ```bash
     redis-cli slowlog get
     ```
   - Optimize configuration based on usage patterns

## GitHub Integration

### Personal Access Token

1. Go to GitHub Settings > Developer Settings > Personal Access Tokens
2. Generate new token with required scopes:
   - `repo` (Full control of private repositories)
   - `read:org` (Read organization and team membership)
   - `read:user` (Read user profile data)

### Webhook Setup

1. Go to repository settings > Webhooks
2. Add new webhook:
   - Payload URL: `http://your-domain/api/v1/webhooks/github`
   - Content type: `application/json`
   - Secret: Generate secure random string
   - Events: Select required events

## Troubleshooting

### Common Issues

1. **Database Connection Failed**

   - Check PostgreSQL service is running
   - Verify database credentials in `.env`
   - Ensure database exists

2. **GitHub API Rate Limits**

   - Check token permissions
   - Implement caching
   - Monitor rate limit headers

3. **CORS Errors**

   - Verify backend CORS configuration
   - Check frontend API URL
   - Ensure proper headers

4. **WebSocket Connection Issues**
   - Check WebSocket URL
   - Verify token authentication
   - Monitor network connectivity

### Debugging Tools

1. **Backend**

   - Enable debug logging: `RUST_LOG=debug cargo run`
   - Use `dbg!()` macro for quick debugging
   - Check PostgreSQL logs

2. **Frontend**
   - Browser Developer Tools
   - React Developer Tools
   - Network tab for API requests

## Production Deployment

### Prerequisites

- VPS with Ubuntu 20.04+
- Domain name with DNS configured
- SSL certificate (Let's Encrypt)

### Deployment Steps

1. **Server Setup**

   ```bash
   # Install required packages
   sudo apt update
   sudo apt install nginx postgresql certbot
   ```

2. **Database Setup**

   ```bash
   sudo -u postgres psql
   CREATE DATABASE github_dashboard;
   CREATE USER dashboard WITH PASSWORD 'secure_password';
   GRANT ALL PRIVILEGES ON DATABASE github_dashboard TO dashboard;
   ```

3. **Backend Deployment**

   ```bash
   # Build release binary
   cargo build --release

   # Copy binary to server
   scp target/release/backend user@server:/opt/github-dashboard/

   # Set up systemd service
   sudo nano /etc/systemd/system/github-dashboard.service
   ```

4. **Frontend Deployment**

   ```bash
   # Build production bundle
   npm run build

   # Copy to server
   scp -r build/* user@server:/var/www/github-dashboard/
   ```

5. **Nginx Configuration**

   ```nginx
   server {
       listen 80;
       server_name dashboard.example.com;

       location /api/ {
           proxy_pass http://localhost:8080;
           proxy_http_version 1.1;
           proxy_set_header Upgrade $http_upgrade;
           proxy_set_header Connection 'upgrade';
           proxy_set_header Host $host;
           proxy_cache_bypass $http_upgrade;
       }

       location / {
           root /var/www/github-dashboard;
           try_files $uri $uri/ /index.html;
       }
   }
   ```

6. **SSL Setup**
   ```bash
   sudo certbot --nginx -d dashboard.example.com
   ```

## Maintenance

### Regular Tasks

1. **Database Backup**

   ```bash
   # Daily backup
   0 0 * * * pg_dump -U postgres github_dashboard > /backups/daily/backup-$(date +\%Y\%m\%d).sql

   # Weekly backup
   0 0 * * 0 pg_dump -U postgres github_dashboard > /backups/weekly/backup-$(date +\%Y\%m\%d).sql
   ```

2. **Log Rotation**

   ```bash
   # Configure logrotate
   sudo nano /etc/logrotate.d/github-dashboard
   ```

3. **System Updates**
   ```bash
   sudo apt update
   sudo apt upgrade
   ```

### Monitoring

1. **System Metrics**

   - CPU usage
   - Memory usage
   - Disk space
   - Network traffic

2. **Application Metrics**

   - API response times
   - Error rates
   - GitHub API rate limits
   - Database performance

3. **Alerting**
   - Set up monitoring alerts
   - Configure notification channels
   - Define alert thresholds
