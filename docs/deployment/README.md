# Deployment Guide

This guide provides instructions for deploying the GitHub Dashboard on a VPS, with a focus on resource efficiency and optimal performance.

## Prerequisites

### VPS Requirements
- Ubuntu 20.04+ or similar Linux distribution
- 1GB RAM minimum (2GB recommended)
- 10GB storage
- Root access or sudo privileges

### Software Requirements
- Nginx
- PostgreSQL
- Node.js (for frontend)
- Rust toolchain (for backend)
- Certbot (for SSL)

## Deployment Steps

### 1. System Setup

```bash
# Update system
sudo apt update
sudo apt upgrade -y

# Install required packages
sudo apt install -y nginx postgresql certbot python3-certbot-nginx
```

### 2. Database Setup

```bash
# Create database and user
sudo -u postgres psql
CREATE DATABASE github_dashboard;
CREATE USER dashboard WITH PASSWORD 'secure_password';
GRANT ALL PRIVILEGES ON DATABASE github_dashboard TO dashboard;
```

### 3. Backend Deployment

```bash
# Build release binary
cd backend
cargo build --release

# Create systemd service
sudo nano /etc/systemd/system/github-dashboard.service
```

Service file content:
```ini
[Unit]
Description=GitHub Dashboard Backend
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/opt/github-dashboard/backend
ExecStart=/opt/github-dashboard/backend/target/release/backend
Restart=always
Environment=RUST_LOG=info
Environment=DATABASE_URL=postgresql://dashboard:secure_password@localhost/github_dashboard

[Install]
WantedBy=multi-user.target
```

### 4. Frontend Deployment

```bash
# Build production bundle
cd frontend
npm install
npm run build

# Copy to Nginx directory
sudo cp -r build/* /var/www/github-dashboard/
```

### 5. Nginx Configuration

```bash
# Create Nginx configuration
sudo nano /etc/nginx/sites-available/github-dashboard
```

Nginx configuration:
```nginx
server {
    listen 80;
    server_name your-domain.com;

    # Frontend
    location / {
        root /var/www/github-dashboard;
        try_files $uri $uri/ /index.html;
    }

    # Backend API
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

### 6. SSL Setup

```bash
# Obtain SSL certificate
sudo certbot --nginx -d your-domain.com
```

## Resource Optimization

### Backend Optimization

1. **Memory Usage**
   - Monitor with `htop`
   - Set appropriate worker count
   - Configure connection pooling

2. **Database Optimization**
   - Regular vacuuming
   - Appropriate indexes
   - Query optimization

3. **Caching Strategy**
   - Implement response caching
   - Use memory-efficient cache
   - Set appropriate TTLs

### Frontend Optimization

1. **Asset Optimization**
   - Enable gzip compression
   - Use CDN for static assets
   - Implement browser caching

2. **Performance Monitoring**
   - Set up error tracking
   - Monitor load times
   - Track resource usage

## Maintenance

### Regular Tasks

1. **System Updates**
   ```bash
   sudo apt update
   sudo apt upgrade
   ```

2. **Database Maintenance**
   ```bash
   sudo -u postgres vacuumdb --analyze github_dashboard
   ```

3. **Log Rotation**
   ```bash
   sudo nano /etc/logrotate.d/github-dashboard
   ```

### Monitoring

1. **Resource Usage**
   - Use `htop` for CPU/memory
   - Monitor disk space
   - Check network usage

2. **Application Health**
   - Check service status
   - Monitor error logs
   - Track response times

## Troubleshooting

### Common Issues

1. **High Resource Usage**
   - Check running processes
   - Review database queries
   - Monitor API calls

2. **Database Issues**
   - Check connection limits
   - Review query performance
   - Monitor disk space

3. **Application Errors**
   - Check service logs
   - Review error tracking
   - Monitor API responses

### Recovery Steps

1. **Service Restart**
   ```bash
   sudo systemctl restart github-dashboard
   ```

2. **Database Recovery**
   ```bash
   sudo -u postgres psql github_dashboard
   ```

3. **Log Analysis**
   ```bash
   sudo journalctl -u github-dashboard
   ```

## Security Considerations

### System Security

1. **Firewall Configuration**
   ```bash
   sudo ufw allow 80
   sudo ufw allow 443
   sudo ufw enable
   ```

2. **Regular Updates**
   - System packages
   - Application dependencies
   - Security patches

### Application Security

1. **Environment Variables**
   - Secure storage
   - Limited access
   - Regular rotation

2. **Access Control**
   - Rate limiting
   - Authentication
   - Authorization

## Conclusion

This deployment guide focuses on efficient resource usage and optimal performance for VPS deployment. Regular maintenance and monitoring are essential for smooth operation.
