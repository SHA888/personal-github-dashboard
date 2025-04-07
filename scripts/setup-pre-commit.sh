#!/bin/bash

# Install pre-commit if not already installed
if ! command -v pre-commit &> /dev/null; then
    echo "Installing pre-commit..."
    sudo apt-get update
    sudo apt-get install -y pre-commit
fi

# Install pre-commit hooks
echo "Installing pre-commit hooks..."
pre-commit install

# Install frontend dependencies if needed
if [ -d "frontend" ]; then
    echo "Installing frontend dependencies..."
    cd frontend
    npm install --legacy-peer-deps
    cd ..
fi

# Install backend dependencies if needed
if [ -d "backend" ]; then
    echo "Installing backend dependencies..."
    cd backend
    rustup component add rustfmt
    rustup component add clippy
    cd ..
fi

echo "Pre-commit hooks setup complete!"
