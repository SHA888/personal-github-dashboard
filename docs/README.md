# Personal GitHub Dashboard Documentation

Welcome to the Personal GitHub Dashboard documentation. This is an open-source project that helps you track and analyze your GitHub activity across repositories and organizations.

## Project Overview

Personal GitHub Dashboard is an open-source project that helps you track and analyze your GitHub activity. It offers:

- **Core Features**:
  - Repository activity tracking and analytics
  - Organization management and insights
  - Real-time updates via WebSocket
  - Custom dashboards and reports
  - Advanced data visualizations
  - Dark/light theme support

## Documentation Structure

- [Vision](./vision/README.md) - Project vision, goals, and roadmap
- [Features](./features/README.md) - Comprehensive feature documentation
- [Setup Guide](./setup/README.md) - Development environment setup instructions
- [Architecture](./architecture/README.md) - System design and technical architecture
- [API Documentation](./api/README.md) - REST API endpoints and WebSocket interface
- [Development Guide](./development/README.md) - Contributing guidelines and best practices
- [Deployment Guide](./deployment/README.md) - Production deployment and maintenance

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

## Quick Start

For a quick start, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/SHA888/personal-github-dashboard.git
   cd personal-github-dashboard
   ```

2. Follow the [Setup Guide](./setup/README.md) for detailed instructions

3. Start the development servers:
   ```bash
   npm run dev
   ```

4. Access the application:
   - Frontend: http://localhost:3001
   - Backend API: http://localhost:3000

## Development Workflow

1. Read the [Development Guide](./development/README.md)
2. Set up your development environment
3. Pick an issue or feature to work on
4. Create a feature branch
5. Follow the coding standards
6. Submit a pull request

## Deployment

For production deployment:

1. Follow the [Deployment Guide](./deployment/README.md)
2. Configure environment variables
3. Set up monitoring and logging
4. Configure backup procedures

## Support

If you encounter any issues:
1. Check the [Troubleshooting](./setup/README.md#troubleshooting) section
2. Search existing GitHub issues
3. Create a new issue with detailed information

## Contributing

We welcome contributions! Please read our [Development Guide](./development/README.md) for:
- Code style guidelines
- Pull request process
- Testing requirements
- Documentation standards

## Project Links

- **GitHub Repository**: [https://github.com/SHA888/personal-github-dashboard](https://github.com/SHA888/personal-github-dashboard)
- **Issues**: [GitHub Issues](https://github.com/SHA888/personal-github-dashboard/issues)
- **Discussions**: [GitHub Discussions](https://github.com/SHA888/personal-github-dashboard/discussions)

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
