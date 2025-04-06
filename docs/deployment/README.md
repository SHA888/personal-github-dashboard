# Deployment Guide

This guide provides instructions for deploying the GitHub Dashboard to production.

## Prerequisites

### Server Requirements
- Ubuntu 20.04 LTS or later
- 2+ CPU cores
- 4GB+ RAM
- 20GB+ storage
- Domain name (optional)

### Required Software
- Nginx
- PostgreSQL
- Node.js
- Rust
- Git
- SSL certificate (Let's Encrypt)

## Deployment Steps

### 1. Server Setup

#### Update System
```bash
sudo apt update
sudo apt upgrade -y
```

#### Install Required Packages
```bash
sudo apt install -y nginx postgresql postgresql-contrib certbot python3-certbot-nginx
```

### 2. Database Setup

#### Create Database and User
```bash
sudo -u postgres psql
```
```sql
CREATE DATABASE github_dashboard;
CREATE USER dashboard_user WITH PASSWORD 'your_secure_password';
GRANT ALL PRIVILEGES ON DATABASE github_dashboard TO dashboard_user;
```

### 3. Application Setup

#### Clone Repository
```bash
git clone https://github.com/SHA888/github-dashboard.git
cd github-dashboard
```

#### Backend Configuration
```bash
cd backend
cp .env.example .env
```

Edit `.env`:
```env
DATABASE_URL=postgres://dashboard_user:your_secure_password@localhost:5432/github_dashboard
GITHUB_TOKEN=your_production_github_token
PORT=8080
RUST_LOG=info
```

#### Frontend Configuration
```bash
cd ../frontend
cp .env.example .env
```

Edit `.env`:
```env
VITE_API_URL=https://your-domain.com/api
```

### 4. Build and Deploy

#### Backend
```bash
cd backend
cargo build --release
sqlx database create
sqlx migrate run
```

#### Frontend
```bash
cd ../frontend
npm install
npm run build
```

### 5. Nginx Configuration

Create `/etc/nginx/sites-available/github-dashboard`:
```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        root /path/to/github-dashboard/frontend/dist;
        try_files $uri $uri/ /index.html;
    }

    location /api {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

Enable site:
```bash
sudo ln -s /etc/nginx/sites-available/github-dashboard /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

### 6. SSL Setup

```bash
sudo certbot --nginx -d your-domain.com
```

### 7. Systemd Service

Create `/etc/systemd/system/github-dashboard.service`:
```ini
[Unit]
Description=GitHub Dashboard Backend
After=network.target

[Service]
User=your-user
WorkingDirectory=/path/to/github-dashboard/backend
ExecStart=/path/to/github-dashboard/backend/target/release/github-dashboard
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable and start service:
```bash
sudo systemctl enable github-dashboard
sudo systemctl start github-dashboard
```

## Monitoring

### Logs
```bash
# Backend logs
sudo journalctl -u github-dashboard -f

# Nginx logs
sudo tail -f /var/log/nginx/error.log
sudo tail -f /var/log/nginx/access.log
```

### Metrics
- Set up Prometheus for metrics
- Configure Grafana dashboards
- Monitor system resources

## Backup and Recovery

### Database Backups
```bash
# Daily backup
pg_dump -U dashboard_user github_dashboard > backup.sql

# Restore
psql -U dashboard_user github_dashboard < backup.sql
```

### Application Backups
- Backup configuration files
- Backup SSL certificates
- Backup database regularly

## Security Considerations

- Keep system updated
- Use strong passwords
- Configure firewall
- Enable automatic security updates
- Regular security audits 