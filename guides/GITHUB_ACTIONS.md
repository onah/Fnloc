# GitHub Actions Setup for Fnloc

This document explains the GitHub Actions workflows configured for the Fnloc project.

## Workflows

### 1. CI Workflow (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `main` branch
- Pull requests to `main` branch

**Jobs:**
- **Test Job (Ubuntu)**: Runs formatting checks, clippy, tests, and build
- **Build Job (Multi-platform)**: Tests builds on Ubuntu, Windows, and macOS

**Features:**
- Code formatting verification (`cargo fmt --check`)
- Linting with clippy (`cargo clippy`)
- Unit and integration tests
- Multi-platform compilation testing
- Cargo dependency caching for faster builds

## Usage

### Running CI
CI runs automatically on every push and pull request to the main branch.

### Local Testing

Before pushing, ensure your code passes all CI checks:

```bash
# Format check
cargo fmt --check

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test

# Build
cargo build --release
```

## Configuration

### Required Secrets
- `GITHUB_TOKEN` - Automatically provided by GitHub Actions

### Cache Strategy
- Cargo registry and build artifacts are cached
- Cache key includes `Cargo.lock` hash for dependency changes
- Separate caches for different platforms

### Build Optimization
- Uses latest stable Rust
- Leverages cargo caching for faster builds
- Multi-platform matrix builds run in parallel

## Troubleshooting

### Common Issues

1. **Formatting Failures**
   ```bash
   cargo fmt
   ```

2. **Clippy Warnings**
   ```bash
   cargo clippy --fix --allow-dirty
   ```

3. **Test Failures**
   ```bash
   cargo test --verbose
   ```

4. **Build Failures**
   - Check `Cargo.toml` dependencies
   - Ensure code compiles on target platforms

### Viewing Logs
- Go to Actions tab in GitHub repository
- Click on the failed workflow run
- Expand the failed job step to see detailed logs

## Monitoring

- All pushes and PRs trigger CI automatically
- Release artifacts are automatically uploaded to GitHub Releases
- Build status badges can be added to README if desired

The workflows are designed to:
- Ensure code quality and consistency
- Test across multiple platforms
- Automate the release process
- Provide fast feedback on changes
