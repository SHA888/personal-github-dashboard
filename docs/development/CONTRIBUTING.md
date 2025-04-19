# Contributing to Personal GitHub Dashboard

Thank you for your interest in contributing to Personal GitHub Dashboard! This document provides guidelines and instructions for contributing to both web and desktop modes of the application.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before contributing.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/personal-github-dashboard.git
   cd personal-github-dashboard
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/original/personal-github-dashboard.git
   ```
4. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

Please refer to [SETUP.md](SETUP.md) for detailed instructions on setting up your development environment.

## Development Workflow

1. **Keep Your Fork Updated**

   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Install Dependencies**

   ```bash
   # Frontend dependencies
   cd frontend
   npm install

   # Backend dependencies
   cd backend
   cargo build
   ```

3. **Run Development Servers**

   ```bash
   # Frontend
   cd frontend
   npm run dev

   # Backend
   cd backend
   cargo run
   ```

## Code Style Guidelines

### Rust Guidelines

- Follow the Rust Style Guide
- Use `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Document public APIs using rustdoc
- Write unit tests for new functionality

### TypeScript/React Guidelines

- Follow the project's ESLint configuration
- Use TypeScript for type safety
- Write functional components with hooks
- Document components using JSDoc
- Follow Material-UI best practices

## Testing

1. **Running Tests**

   ```bash
   # Frontend tests
   cd frontend
   npm test

   # Backend tests
   cd backend
   cargo test
   ```

2. **Writing Tests**
   - Write unit tests for new functionality
   - Include integration tests for API endpoints
   - Add E2E tests for critical user flows
   - Test both success and error cases

## Pull Request Process

1. **Before Submitting**

   - Update documentation for new features
   - Add tests for new functionality
   - Run the full test suite
   - Update the changelog
   - Rebase on latest main

2. **PR Guidelines**

   - Use the PR template
   - Link related issues
   - Include screenshots for UI changes
   - List breaking changes
   - Update dependencies if needed

3. **Review Process**
   - Address review comments
   - Keep the PR focused and small
   - Squash commits before merging
   - Maintain a clean commit history

## Feature Flags

- Use feature flags for new functionality
- Document flags in [FEATURES.md](FEATURES.md)
- Test both enabled and disabled states

## Documentation

1. **Code Documentation**

   - Document public APIs
   - Add inline comments for complex logic
   - Update README.md when needed
   - Keep documentation up to date

2. **User Documentation**
   - Update user guides for new features
   - Add examples and screenshots
   - Document configuration options
   - Include troubleshooting steps

## Release Process

1. **Version Bumping**

   - Follow semantic versioning
   - Update version in Cargo.toml
   - Update version in package.json
   - Update CHANGELOG.md

2. **Release Checklist**
   - Run full test suite
   - Update documentation
   - Create release notes
   - Tag the release
   - Update deployment configs

## Reporting Issues

1. **Bug Reports**

   - Use the bug report template
   - Include reproduction steps
   - Attach relevant logs
   - Specify environment details

2. **Feature Requests**
   - Use the feature request template
   - Explain the use case
   - Suggest implementation details
   - Consider alternatives

## Community

- Join our Discord server
- Participate in discussions
- Help other contributors
- Share your experience

## License

By contributing, you agree that your contributions will be licensed under the project's MIT License.

## Questions?

Feel free to:

- Open an issue
- Join our Discord
- Contact maintainers
- Check the FAQ

## Additional Resources

- [Project Architecture](../architecture/README.md)
- [API Documentation](../api/README.md)
- [Development Setup](SETUP.md)
- [Feature Documentation](FEATURES.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [License](../../LICENSE)
