# 🤝 Contributing to RustSat-ESA

Thank you for your interest in contributing to RustSat-ESA! This document provides guidelines for contributing to this space communication protocol stack project.

## 🌟 Ways to Contribute

### Code Contributions
- **Bug fixes**: Fix issues in existing functionality
- **New features**: Implement additional space protocols or capabilities
- **Performance improvements**: Optimize critical code paths
- **Documentation**: Improve code documentation and examples
- **Testing**: Add unit tests, integration tests, or benchmarks

### Non-Code Contributions
- **Documentation**: Write tutorials, guides, or improve existing docs
- **Issue reporting**: Report bugs or suggest enhancements
- **Community support**: Help other users in discussions
- **Hardware testing**: Test with real CubeSat hardware
- **Mission validation**: Validate with real-world space missions

## 🚀 Getting Started

### Development Environment Setup

1. **Install Prerequisites**:
   ```bash
   # Install Rust (latest stable)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   
   # Install development tools
   cargo install cargo-watch cargo-tarpaulin cargo-audit
   rustup component add clippy rustfmt
   ```

2. **Fork and Clone**:
   ```bash
   # Fork the repository on GitHub
   # Then clone your fork
   git clone https://github.com/s7g4/rustsat-esa.git
   cd rustsat-esa
   
   # Add upstream remote
   git remote add upstream https://github.com/s7g4/rustsat-esa.git
   ```

3. **Build and Test**:
   ```bash
   # Build the project
   cargo build
   
   # Run tests
   cargo test
   
   # Run examples
   cargo run --example demo
   ```

### Development Workflow

1. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/issue-number
   ```

2. **Make changes**:
   - Write code following the project's style guidelines
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**:
   ```bash
   # Run all tests
   cargo test
   
   # Check formatting
   cargo fmt --check
   
   # Run linter
   cargo clippy -- -D warnings
   
   # Check for security issues
   cargo audit
   ```

4. **Commit and push**:
   ```bash
   git add .
   git commit -m "feat: add new SpaceCAN feature"
   git push origin feature/your-feature-name
   ```

5. **Create Pull Request**:
   - Go to GitHub and create a pull request
   - Fill out the pull request template
   - Wait for review and address feedback

## 📝 Code Style Guidelines

### Rust Style
- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `cargo fmt` for automatic formatting
- Use `cargo clippy` for linting
- Maximum line length: 100 characters

### Code Organization
```rust
// File structure example
use std::collections::HashMap;
use std::error::Error;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::protocol::spacecan::SpaceCANFrame;

/// Documentation for public structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleStruct {
    /// Field documentation
    pub field: String,
    private_field: u32,
}

impl ExampleStruct {
    /// Constructor documentation
    pub fn new(field: String) -> Self {
        Self {
            field,
            private_field: 0,
        }
    }
    
    /// Method documentation
    pub fn process(&mut self) -> Result<(), Box<dyn Error>> {
        // Implementation
        Ok(())
    }
}
```

### Documentation Standards
- All public APIs must have documentation
- Use `///` for doc comments
- Include examples in documentation when helpful
- Document error conditions and panics

```rust
/// Encodes a SpaceCAN frame for transmission.
///
/// This function takes a SpaceCAN frame and converts it to a byte vector
/// suitable for transmission over a communication link.
///
/// # Arguments
///
/// * `frame` - The SpaceCAN frame to encode
///
/// # Returns
///
/// Returns a `Vec<u8>` containing the encoded frame data.
///
/// # Examples
///
/// ```rust
/// use rustsat_esa::protocol::spacecan::{SpaceCANFrame, FramePriority};
///
/// let frame = SpaceCANFrame::new(0x123, vec![1, 2, 3, 4], FramePriority::High);
/// let encoded = frame.encode();
/// assert!(!encoded.is_empty());
/// ```
///
/// # Errors
///
/// This function currently does not return errors, but future versions
/// may return encoding errors for invalid frame data.
pub fn encode(&self) -> Vec<u8> {
    // Implementation
}
```

### Testing Guidelines
- Write unit tests for all public functions
- Use integration tests for end-to-end scenarios
- Include property-based tests for complex algorithms
- Aim for >90% test coverage

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_encoding() {
        let frame = SpaceCANFrame::new(0x123, vec![1, 2, 3, 4], FramePriority::High);
        let encoded = frame.encode();
        
        assert!(!encoded.is_empty());
        assert_eq!(encoded.len(), 32); // Expected frame size
    }

    #[test]
    fn test_frame_roundtrip() {
        let original = SpaceCANFrame::new(0x456, vec![5, 6, 7, 8], FramePriority::Normal);
        let encoded = original.encode();
        let decoded = SpaceCANFrame::decode(&encoded).unwrap();
        
        assert_eq!(original.id, decoded.id);
        assert_eq!(original.data, decoded.data);
        assert_eq!(original.priority, decoded.priority);
    }
}
```

## 🐛 Issue Reporting

### Bug Reports
When reporting bugs, please include:

1. **Environment information**:
   - Operating system and version
   - Rust version (`rustc --version`)
   - Project version or commit hash

2. **Steps to reproduce**:
   - Minimal code example
   - Command line arguments used
   - Expected vs. actual behavior

3. **Additional context**:
   - Error messages or logs
   - Screenshots if applicable
   - Related issues or pull requests

### Feature Requests
When suggesting new features:

1. **Use case description**: Explain why this feature would be useful
2. **Proposed solution**: Describe how you envision the feature working
3. **Alternatives considered**: Mention other approaches you've considered
4. **Implementation notes**: Technical details if you have ideas

## 🔍 Code Review Process

### For Contributors
- Keep pull requests focused and reasonably sized
- Write clear commit messages following [Conventional Commits](https://www.conventionalcommits.org/)
- Respond to review feedback promptly
- Update your branch with latest changes from main before requesting review

### Commit Message Format
```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(spacecan): add Reed-Solomon error correction

Implements RS(255,223) error correction for SpaceCAN frames to improve
reliability in noisy space environments.

Closes #123
```

### Review Criteria
Reviewers will check for:
- **Correctness**: Does the code work as intended?
- **Performance**: Are there any performance regressions?
- **Security**: Are there any security implications?
- **Style**: Does the code follow project conventions?
- **Tests**: Are there adequate tests for the changes?
- **Documentation**: Is the code properly documented?

## 🏗️ Architecture Guidelines

### Module Organization
```
src/
├── protocol/           # Communication protocols
│   ├── spacecan.rs    # SpaceCAN implementation
│   ├── network.rs     # Mesh networking
│   └── mod.rs         # Module exports
├── cubesat/           # Satellite operations
├── security/          # Cryptography and security
├── telemetry/         # Data collection and processing
├── simulation/        # Space environment simulation
├── ground_station/    # Ground network integration
├── web/              # Web dashboard
├── metrics/          # Performance monitoring
├── config/           # Configuration management
└── lib.rs            # Main library interface
```

### Design Principles
1. **Modularity**: Each module should have a clear, single responsibility
2. **Testability**: Code should be easy to unit test
3. **Performance**: Critical paths should be optimized
4. **Safety**: Use Rust's type system to prevent errors
5. **Documentation**: Public APIs should be well-documented

### Error Handling
- Use `Result<T, E>` for recoverable errors
- Use `panic!` only for unrecoverable errors
- Create custom error types for domain-specific errors
- Provide meaningful error messages

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpaceCANError {
    #[error("Invalid frame ID: {id}")]
    InvalidFrameId { id: u32 },
    
    #[error("Frame too large: {size} bytes (max: {max})")]
    FrameTooLarge { size: usize, max: usize },
    
    #[error("CRC mismatch: expected {expected:08x}, got {actual:08x}")]
    CrcMismatch { expected: u32, actual: u32 },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

## 🧪 Testing Strategy

### Test Categories
1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test component interactions
3. **End-to-End Tests**: Test complete workflows
4. **Performance Tests**: Benchmark critical operations
5. **Property Tests**: Test invariants with random inputs

### Test Organization
```
tests/
├── integration_tests.rs    # Integration tests
├── performance_tests.rs    # Performance benchmarks
└── common/                 # Shared test utilities
    ├── mod.rs
    └── test_utils.rs
```

### Continuous Integration
All pull requests must pass:
- Compilation on stable Rust
- All unit and integration tests
- Code formatting check (`cargo fmt --check`)
- Linting check (`cargo clippy -- -D warnings`)
- Security audit (`cargo audit`)

## 📚 Documentation

### Types of Documentation
1. **API Documentation**: Inline code documentation
2. **User Guides**: How-to guides and tutorials
3. **Architecture Documentation**: System design and structure
4. **Examples**: Working code examples

### Documentation Tools
- Use `cargo doc` to generate API documentation
- Write examples in `examples/` directory
- Create markdown files for guides and tutorials
- Keep README.md up to date

## 🌍 Community Guidelines

### Code of Conduct
- Be respectful and inclusive
- Focus on constructive feedback
- Help newcomers learn and contribute
- Maintain a professional tone in all interactions

### Communication Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and discussions
- **Pull Requests**: Code review and collaboration

## 🎯 Priority Areas

We especially welcome contributions in these areas:

### High Priority
- **Hardware Integration**: Testing with real CubeSat hardware
- **Performance Optimization**: Critical path optimizations
- **Security Auditing**: Security review and improvements
- **Documentation**: User guides and tutorials

### Medium Priority
- **Additional Protocols**: CCSDS, AX.25, other space protocols
- **Advanced Features**: Machine learning, AI-based optimizations
- **Visualization**: Enhanced dashboard features
- **Mobile Support**: Mobile-friendly interfaces

### Low Priority
- **Code Cleanup**: Refactoring and code organization
- **Additional Tests**: Expanding test coverage
- **Tooling**: Development and build tools
- **Examples**: More usage examples

## 📄 License

By contributing to RustSat-ESA, you agree that your contributions will be licensed under the same MIT License that covers the project.

## 🙏 Recognition

Contributors will be recognized in:
- The project's README.md file
- Release notes for significant contributions
- The project's contributors page

Thank you for helping make RustSat-ESA better! 🚀