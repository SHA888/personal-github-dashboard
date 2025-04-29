# Deployment Guide

This guide provides comprehensive instructions for deploying the Personal GitHub Dashboard in a production environment.

## Prerequisites

- Docker and Docker Compose
- Domain name with SSL certificate
- PostgreSQL 15+ database server
- Redis 7+ server
- Node.js 18+ (for build process)
- Rust 1.75+ (for build process)

## Environment Configuration

### Backend Environment Variables

Create a `.env` file in the backend directory:

```env
# Server Configuration
PORT=3000
HOST=0.0.0.0
RUST_LOG=info
RUST_BACKTRACE=1

# Database Configuration
DATABASE_URL=postgresql://user:password@localhost:5432/personal_github_dashboard
DATABASE_POOL_SIZE=5

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_POOL_SIZE=5

# GitHub Configuration
GITHUB_CLIENT_ID=your_client_id
GITHUB_CLIENT_SECRET=your_client_secret
GITHUB_REDIRECT_URL=https://your-domain.com/api/auth/github/callback

# Security
JWT_SECRET=your_secure_jwt_secret
CORS_ORIGIN=https://your-domain.com
```

### Frontend Environment Variables

Create a `.env` file in the frontend directory:

```env
VITE_API_URL=https://your-domain.com/api
VITE_WS_URL=wss://your-domain.com/ws
VITE_GITHUB_CLIENT_ID=your_client_id
```

## Build Process

1. Build the frontend:

   ```bash
   cd frontend
   npm install
   npm run build
   ```

2. Build the backend:
   ```bash
   cd backend
   cargo build --release
   ```

## Database Setup

1. Create the production database:

   ```bash
   sqlx database create
   ```

2. Run migrations:
   ```bash
   sqlx migrate run
   ```

## Docker Deployment

1. Build the Docker images:

   ```bash
   docker-compose -f docker-compose.prod.yml build
   ```

2. Start the services:
   ```bash
   docker-compose -f docker-compose.prod.yml up -d
   ```

## Nginx Configuration

Example Nginx configuration for reverse proxy:

```nginx
server {
    listen 443 ssl http2;
    server_name your-domain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    # Frontend
    location / {
        root /var/www/personal-github-dashboard;
        try_files $uri $uri/ /index.html;
    }

    # Backend API
    location /api {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    # WebSocket
    location /ws {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "Upgrade";
        proxy_set_header Host $host;
    }
}
```

## Monitoring & Logging

### Application Monitoring

1. Set up Prometheus metrics endpoint:

   - Backend metrics available at `/metrics`
   - Configure Prometheus to scrape this endpoint

2. Configure logging:
   - Backend logs to stdout/stderr
   - Use docker logging driver or external logging service

### Health Checks

- Backend health check endpoint: `/api/health`
- Database health check: `/api/health/db`
- Redis health check: `/api/health/redis`

## Backup Procedures

### Database Backups

1. Automated daily backups:

   ```bash
   pg_dump -U user personal_github_dashboard > backup_$(date +%Y%m%d).sql
   ```

2. Configure backup retention policy:
   - Keep daily backups for 7 days
   - Keep weekly backups for 1 month
   - Keep monthly backups for 1 year

### Application Data

1. Back up environment configuration files
2. Back up SSL certificates
3. Back up custom configurations

## Security Considerations

1. Enable rate limiting:

   - Configure in `backend/config/rate_limit.rs`
   - Adjust limits based on usage patterns

2. Set up firewalls:

   - Allow only necessary ports (80, 443)
   - Restrict database access

3. Regular updates:
   - Keep dependencies updated
   - Apply security patches promptly

## Scaling Considerations

### Horizontal Scaling

1. Backend scaling:

   - Use load balancer
   - Configure session sharing
   - Scale API servers independently

2. Database scaling:

   - Configure read replicas
   - Consider sharding for large datasets

3. Redis scaling:
   - Set up Redis cluster
   - Configure persistence

## Maintenance Procedures

### Regular Maintenance

1. Monitor system resources:

   - CPU usage
   - Memory usage
   - Disk space
   - Network bandwidth

2. Clean up old data:
   - Archive old logs
   - Remove unused Docker images
   - Clean up temporary files

### Updating the Application

1. Create backup before updating
2. Follow semantic versioning
3. Test updates in staging environment
4. Use blue-green deployment for zero downtime

## Troubleshooting

### Common Issues

1. Connection issues:

   - Check network connectivity
   - Verify DNS settings
   - Check firewall rules

2. Performance issues:

   - Monitor database queries
   - Check Redis cache hit rate
   - Review application logs

3. Memory issues:
   - Check container limits
   - Monitor memory leaks
   - Review garbage collection

## Support and Maintenance

For production support:

1. Check the [GitHub Issues](https://github.com/SHA888/personal-github-dashboard/issues)
2. Review error logs
3. Contact maintainers for critical issues
