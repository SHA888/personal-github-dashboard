# Deployment Guide

This guide provides comprehensive instructions for deploying the GitHub Dashboard to production.

## Prerequisites

### Server Requirements
- Ubuntu 20.04 LTS or later
- 2+ CPU cores
- 4GB+ RAM
- 20GB+ storage
- Static IP address
- Domain name with DNS configured

### Required Software
1. **System Packages**
   ```bash
   sudo apt update
   sudo apt install -y \
     nginx \
     postgresql \
     certbot \
     python3-certbot-nginx \
     build-essential \
     pkg-config \
     libssl-dev
   ```

2. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup default stable
   ```

3. **Node.js**
   ```bash
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt install -y nodejs
   ```

## Server Setup

### 1. System Configuration

1. **Update System**
   ```bash
   sudo apt update
   sudo apt upgrade -y
   sudo apt autoremove -y
   ```

2. **Configure Firewall**
   ```bash
   sudo ufw allow ssh
   sudo ufw allow http
   sudo ufw allow https
   sudo ufw enable
   ```

3. **Create Deployment User**
   ```bash
   sudo adduser deployer
   sudo usermod -aG sudo deployer
   ```

### 2. Database Setup

1. **Configure PostgreSQL**
   ```bash
   sudo -u postgres psql
   CREATE DATABASE github_dashboard;
   CREATE USER dashboard WITH PASSWORD 'secure_password';
   GRANT ALL PRIVILEGES ON DATABASE github_dashboard TO dashboard;
   ```

2. **Configure PostgreSQL Access**
   ```bash
   sudo nano /etc/postgresql/12/main/pg_hba.conf
   # Add line:
   local   github_dashboard    dashboard    md5
   ```

3. **Restart PostgreSQL**
   ```bash
   sudo systemctl restart postgresql
   ```

### 3. Application Setup

1. **Create Application Directory**
   ```bash
   sudo mkdir -p /opt/github-dashboard
   sudo chown -R deployer:deployer /opt/github-dashboard
   ```

2. **Clone Repository**
   ```bash
   cd /opt/github-dashboard
   git clone https://github.com/yourusername/github-dashboard.git .
   ```

3. **Configure Environment**
   ```bash
   # Backend
   nano backend/.env
   GITHUB_TOKEN=your_github_pat
   DATABASE_URL=postgres://dashboard:secure_password@localhost:5432/github_dashboard
   PORT=8080
   RUST_LOG=info
   
   # Frontend
   nano frontend/.env
   REACT_APP_API_URL=https://dashboard.example.com/api/v1
   REACT_APP_WS_URL=wss://dashboard.example.com/api/v1/ws
   ```

### 4. Backend Deployment

1. **Build Backend**
   ```bash
   cd /opt/github-dashboard/backend
   cargo build --release
   ```

2. **Create Systemd Service**
   ```bash
   sudo nano /etc/systemd/system/github-dashboard.service
   ```
   ```ini
   [Unit]
   Description=GitHub Dashboard Backend
   After=network.target postgresql.service
   
   [Service]
   User=deployer
   Group=deployer
   WorkingDirectory=/opt/github-dashboard/backend
   Environment="RUST_LOG=info"
   EnvironmentFile=/opt/github-dashboard/backend/.env
   ExecStart=/opt/github-dashboard/backend/target/release/backend
   Restart=always
   
   [Install]
   WantedBy=multi-user.target
   ```

3. **Start Backend Service**
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable github-dashboard
   sudo systemctl start github-dashboard
   ```

### 5. Frontend Deployment

1. **Build Frontend**
   ```bash
   cd /opt/github-dashboard/frontend
   npm install
   npm run build
   ```

2. **Configure Nginx**
   ```bash
   sudo nano /etc/nginx/sites-available/github-dashboard
   ```
   ```nginx
   server {
       listen 80;
       server_name dashboard.example.com;
       
       root /opt/github-dashboard/frontend/build;
       index index.html;
       
       location /api/ {
           proxy_pass http://localhost:8080;
           proxy_http_version 1.1;
           proxy_set_header Upgrade $http_upgrade;
           proxy_set_header Connection 'upgrade';
           proxy_set_header Host $host;
           proxy_cache_bypass $http_upgrade;
       }
       
       location / {
           try_files $uri $uri/ /index.html;
       }
   }
   ```

3. **Enable Site**
   ```bash
   sudo ln -s /etc/nginx/sites-available/github-dashboard /etc/nginx/sites-enabled/
   sudo nginx -t
   sudo systemctl restart nginx
   ```

### 6. SSL Setup

1. **Obtain SSL Certificate**
   ```bash
   sudo certbot --nginx -d dashboard.example.com
   ```

2. **Configure Automatic Renewal**
   ```bash
   sudo certbot renew --dry-run
   ```

## Monitoring Setup

### 1. System Monitoring

1. **Install Monitoring Tools**
   ```bash
   sudo apt install -y prometheus node-exporter
   ```

2. **Configure Prometheus**
   ```bash
   sudo nano /etc/prometheus/prometheus.yml
   ```
   ```yaml
   global:
     scrape_interval: 15s
   
   scrape_configs:
     - job_name: 'node'
       static_configs:
         - targets: ['localhost:9100']
   ```

3. **Start Services**
   ```bash
   sudo systemctl enable prometheus node-exporter
   sudo systemctl start prometheus node-exporter
   ```

### 2. Application Monitoring

1. **Configure Logging**
   ```bash
   sudo nano /etc/logrotate.d/github-dashboard
   ```
   ```
   /opt/github-dashboard/backend/logs/*.log {
       daily
       missingok
       rotate 14
       compress
       delaycompress
       notifempty
       create 0640 deployer deployer
   }
   ```

2. **Set Up Alerts**
   ```bash
   sudo apt install -y alertmanager
   sudo nano /etc/alertmanager/alertmanager.yml
   ```

## Backup Strategy

### 1. Database Backups

1. **Daily Backup Script**
   ```bash
   sudo nano /usr/local/bin/backup-database.sh
   ```
   ```bash
   #!/bin/bash
   BACKUP_DIR="/backups/database"
   DATE=$(date +%Y%m%d)
   
   mkdir -p $BACKUP_DIR
   pg_dump -U dashboard github_dashboard > $BACKUP_DIR/backup-$DATE.sql
   gzip $BACKUP_DIR/backup-$DATE.sql
   
   # Keep only last 7 days
   find $BACKUP_DIR -type f -mtime +7 -delete
   ```

2. **Schedule Backup**
   ```bash
   sudo chmod +x /usr/local/bin/backup-database.sh
   sudo crontab -e
   ```
   ```
   0 0 * * * /usr/local/bin/backup-database.sh
   ```

### 2. Application Backups

1. **Backup Script**
   ```bash
   sudo nano /usr/local/bin/backup-application.sh
   ```
   ```bash
   #!/bin/bash
   BACKUP_DIR="/backups/application"
   DATE=$(date +%Y%m%d)
   
   mkdir -p $BACKUP_DIR
   tar -czf $BACKUP_DIR/app-$DATE.tar.gz /opt/github-dashboard
   
   # Keep only last 7 days
   find $BACKUP_DIR -type f -mtime +7 -delete
   ```

2. **Schedule Backup**
   ```bash
   sudo chmod +x /usr/local/bin/backup-application.sh
   sudo crontab -e
   ```
   ```
   0 1 * * * /usr/local/bin/backup-application.sh
   ```

## Maintenance

### 1. Regular Updates

1. **System Updates**
   ```bash
   sudo apt update
   sudo apt upgrade -y
   sudo apt autoremove -y
   ```

2. **Application Updates**
   ```bash
   cd /opt/github-dashboard
   git pull
   
   # Backend
   cd backend
   cargo build --release
   sudo systemctl restart github-dashboard
   
   # Frontend
   cd ../frontend
   npm install
   npm run build
   sudo systemctl restart nginx
   ```

### 2. Monitoring Checks

1. **System Health**
   ```bash
   # Check disk space
   df -h
   
   # Check memory usage
   free -h
   
   # Check running services
   sudo systemctl status github-dashboard nginx postgresql
   ```

2. **Application Health**
   ```bash
   # Check logs
   sudo journalctl -u github-dashboard -f
   
   # Check database
   sudo -u postgres psql -d github_dashboard -c "SELECT count(*) FROM repositories;"
   ```

### 3. Security Updates

1. **Regular Security Checks**
   ```bash
   # Check for security updates
   sudo apt update
   sudo apt list --upgradable
   
   # Check for vulnerable packages
   sudo apt install -y debsecan
   debsecan
   ```

2. **Application Security**
   - Regular dependency updates
   - Security audits
   - Penetration testing
   - Access log review

## Troubleshooting

### 1. Common Issues

1. **Database Connection Issues**
   ```bash
   # Check PostgreSQL status
   sudo systemctl status postgresql
   
   # Check connection
   psql -U dashboard -d github_dashboard
   
   # Check logs
   sudo tail -f /var/log/postgresql/postgresql-12-main.log
   ```

2. **Application Issues**
   ```bash
   # Check application logs
   sudo journalctl -u github-dashboard -f
   
   # Check nginx logs
   sudo tail -f /var/log/nginx/error.log
   
   # Check system resources
   htop
   ```

3. **SSL Issues**
   ```bash
   # Check certificate
   sudo certbot certificates
   
   # Test SSL
   curl -vI https://dashboard.example.com
   ```

### 2. Recovery Procedures

1. **Database Recovery**
   ```bash
   # Restore from backup
   gunzip -c /backups/database/backup-20240406.sql.gz | psql -U dashboard github_dashboard
   ```

2. **Application Recovery**
   ```bash
   # Restore from backup
   tar -xzf /backups/application/app-20240406.tar.gz -C /
   
   # Restart services
   sudo systemctl restart github-dashboard nginx
   ```

3. **SSL Certificate Renewal**
   ```bash
   # Force renewal
   sudo certbot renew --force-renewal
   
   # Check renewal
   sudo certbot certificates
   ``` 