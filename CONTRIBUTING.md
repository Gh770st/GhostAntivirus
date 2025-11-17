# 🤝 Contributing to GhostAntivirus

Thank you for your interest in contributing to GhostAntivirus! This guide will help you get started with contributing to this open-source antivirus project.

## 📋 Table of Contents

- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Testing Guidelines](#testing-guidelines)
- [Security Guidelines](#security-guidelines)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Community](#community)

---

## 🔧 Prerequisites

### Required Tools
- **Rust**: 1.91.0 or later
- **Python**: 3.9 or later
- **Go**: 1.21 or later
- **Node.js**: 18.0 or later
- **Flutter**: 3.0 or later
- **Docker**: Latest stable version

### Development Environment
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python dependencies
pip install -r ai-engine/requirements.txt

# Install Go modules
cd network-guard && go mod download

# Install Node.js dependencies
cd web-ui && npm install

# Install Flutter dependencies
cd mobile-app && flutter pub get
```

---

## 🚀 Getting Started

### 1. Fork and Clone
```bash
# Fork the repository on GitHub
git clone https://github.com/YOUR_USERNAME/GhostAntivirus.git
cd GhostAntivirus
```

### 2. Setup Development Environment
```bash
# Install all dependencies
./scripts/setup-dev.sh

# Run tests to verify setup
./scripts/run-all-tests.sh
```

### 3. Create Development Branch
```bash
git checkout -b feature/your-feature-name
```

---

## 🔄 Development Workflow

### 1. Understand the Architecture
Read the [ARCHITECTURE.md](ARCHITECTURE.md) to understand the system design and component interactions.

### 2. Choose an Area to Contribute
- **Core Engine** (Rust): Scanning, quarantine, AI integration
- **AI Engine** (Python): Machine learning models, threat analysis
- **Network Guard** (Go): Network monitoring, firewall rules
- **Web Dashboard** (TypeScript/React): User interface, real-time updates
- **Browser Extension** (JavaScript): Web protection, safe browsing
- **Mobile App** (Flutter): Mobile security, cross-platform UI

### 3. Development Steps
```bash
# 1. Create feature branch
git checkout -b feature/your-feature

# 2. Make your changes
# Edit files...

# 3. Run tests
cargo test                    # Rust tests
python -m pytest ai-engine/   # Python tests
go test ./...                 # Go tests
npm test                      # Node.js tests
flutter test                  # Flutter tests

# 4. Run security validation
cargo test --test security_validation

# 5. Commit changes
git add .
git commit -m "feat: add your feature description"

# 6. Push and create PR
git push origin feature/your-feature
```

---

## 📝 Code Standards

### Rust Code Standards
```rust
// Use rustfmt for formatting
cargo fmt

// Use clippy for linting
cargo clippy -- -D warnings

// Example of good Rust code
use anyhow::Result;
use log::{info, warn};

pub struct Scanner {
    scan_queue: Arc<Mutex<VecDeque<PathBuf>>>,
    is_scanning: AtomicBool,
}

impl Scanner {
    pub async fn start_scan(&self, path: &Path) -> Result<String> {
        info!("Starting scan on: {:?}", path);
        // Implementation...
    }
}
```

### Python Code Standards
```python
# Use black for formatting
black ai-engine/

# Use flake8 for linting
flake8 ai-engine/

# Example of good Python code
import logging
from typing import List, Optional
from dataclasses import dataclass

logger = logging.getLogger(__name__)

@dataclass
class ThreatInfo:
    name: str
    severity: str
    confidence: float

async def analyze_threat(file_path: str) -> Optional[ThreatInfo]:
    """Analyze file for potential threats."""
    logger.info(f"Analyzing: {file_path}")
    # Implementation...
```

### Go Code Standards
```go
// Use gofmt for formatting
go fmt ./...

// Use golint for linting
golint ./...

// Example of good Go code
package network

import (
    "context"
    "log"
    "sync"
)

type ConnectionMonitor struct {
    connections map[string]Connection
    mu          sync.RWMutex
}

func (cm *ConnectionMonitor) StartMonitoring(ctx context.Context) error {
    log.Println("Starting connection monitoring")
    // Implementation...
}
```

### TypeScript/JavaScript Standards
```typescript
// Use Prettier for formatting
npm run format

// Use ESLint for linting
npm run lint

// Example of good TypeScript code
interface ScanResult {
  id: string;
  threats: ThreatInfo[];
  scannedAt: Date;
}

class ScannerService {
  async startScan(path: string): Promise<ScanResult> {
    console.log(`Starting scan on: ${path}`);
    // Implementation...
  }
}
```

---

## 🧪 Testing Guidelines

### Test Requirements
1. **Unit Tests**: All new functions must have unit tests
2. **Integration Tests**: New features must have integration tests
3. **Security Tests**: Security-related changes must pass security validation
4. **Performance Tests**: Performance-critical code must have benchmarks

### Test Structure
```rust
// Rust tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scanner_creation() {
        let scanner = Scanner::new().await;
        assert!(scanner.is_ok());
    }

    #[test]
    fn test_threat_detection() {
        // Test implementation...
    }
}
```

```python
# Python tests
import pytest
from unittest.mock import Mock, patch

class TestThreatAnalyzer:
    def test_analyze_file(self):
        analyzer = ThreatAnalyzer()
        result = analyzer.analyze_file("test.txt")
        assert result is not None

    @pytest.mark.asyncio
    async def test_async_analysis(self):
        # Async test implementation...
```

### Running Tests
```bash
# Run all tests
./scripts/run-all-tests.sh

# Run specific component tests
cargo test                    # Core engine
python -m pytest ai-engine/   # AI engine
go test ./...                 # Network guard
npm test                      # Web dashboard
flutter test                  # Mobile app

# Run security tests
cargo test --test security_validation

# Run performance benchmarks
cargo bench
```

---

## 🔒 Security Guidelines

### Security Requirements
1. **Input Validation**: All inputs must be validated and sanitized
2. **Error Handling**: No sensitive information in error messages
3. **Memory Safety**: No memory leaks or unsafe operations
4. **Authentication**: Proper authentication and authorization
5. **Rate Limiting**: Implement rate limiting for APIs

### Security Checklist
- [ ] Input validation implemented
- [ ] Error messages don't leak sensitive information
- [ ] No hardcoded secrets or keys
- [ ] Proper access controls implemented
- [ ] Rate limiting configured
- [ ] Security tests pass

### Reporting Security Issues
If you discover a security vulnerability, please report it privately:

1. **Do not** open a public issue
2. Send an email to: security@ghostantivirus.com
3. Include detailed description and reproduction steps
4. We'll respond within 48 hours

---

## 📚 Documentation

### Documentation Requirements
1. **Code Comments**: Public APIs must be documented
2. **README Updates**: New features should be documented in README
3. **API Documentation**: Keep API docs up to date
4. **User Documentation**: Update user guides for new features

### Documentation Standards
```rust
/// Scans a file or directory for threats.
///
/// # Arguments
/// * `path` - Path to scan
/// * `scan_type` - Type of scan (Quick, Full, Custom)
///
/// # Returns
/// * `Result<String>` - Scan ID if successful
///
/// # Examples
/// ```
/// let scanner = Scanner::new().await?;
/// let scan_id = scanner.start_scan("/path/to/scan", ScanType::Quick).await?;
/// ```
pub async fn start_scan(&self, path: &str, scan_type: ScanType) -> Result<String> {
    // Implementation...
}
```

---

## 🔄 Pull Request Process

### Before Submitting PR
1. **Run all tests**: Ensure all tests pass
2. **Code formatting**: Run formatters and linters
3. **Documentation**: Update relevant documentation
4. **Security checks**: Ensure security tests pass
5. **Performance**: Verify no performance regressions

### PR Template
```markdown
## Description
Brief description of changes made.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Security tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project standards
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes (or clearly documented)
```

### PR Review Process
1. **Automated Checks**: CI/CD pipeline runs tests
2. **Code Review**: Maintainers review code quality
3. **Security Review**: Security team validates changes
4. **Approval**: At least one maintainer approval required
5. **Merge**: PR merged after all checks pass

---

## 🏷️ Commit Guidelines

### Commit Message Format
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Maintenance tasks
- `security`: Security-related changes

### Examples
```
feat(scanner): add real-time threat detection

Add real-time threat detection using machine learning
models to improve scan accuracy and response time.

Closes #123
```

```
fix(api): resolve memory leak in scan results

Fix memory leak in scan results handling that caused
gradual memory increase during long-running scans.

Security: potential DoS vulnerability
```

---

## 🌟 Community

### Communication Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and discussions
- **Discord**: Real-time chat with community
- **Twitter**: Updates and announcements

### Code of Conduct
We are committed to providing a welcoming and inclusive environment. Please read our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for more information.

### Getting Help
- **Documentation**: Check existing docs first
- **Issues**: Search existing issues before creating new ones
- **Discussions**: Use GitHub Discussions for questions
- **Discord**: Join our Discord community for real-time help

---

## 🏆 Recognition

### Contributor Recognition
- **Hall of Fame**: Contributors listed in README
- **Monthly Highlights**: Outstanding contributions featured
- **Swag**: Contributors receive project swag
- **Recognition**: Annual contributor awards

### Ways to Contribute
- **Code**: New features, bug fixes, improvements
- **Documentation**: Docs, guides, tutorials
- **Testing**: Test cases, bug reports, validation
- **Translation**: Help translate the project
- **Community**: Support others, share knowledge

---

## 📞 Contact

### Project Maintainers
- **Lead Developer**: [Lead Dev](mailto:lead@ghostantivirus.com)
- **Security Team**: [Security](mailto:security@ghostantivirus.com)
- **Community Manager**: [Community](mailto:community@ghostantivirus.com)

### Social Media
- **Twitter**: [@GhostAntivirus](https://twitter.com/GhostAntivirus)
- **GitHub**: [GhostAntivirus](https://github.com/Gh770st/GhostAntivirus)
- **Website**: [ghostantivirus.com](https://ghostantivirus.com)

---

## 🎉 Thank You!

Thank you for contributing to GhostAntivirus! Your contributions help make the digital world safer for everyone.

Every contribution, no matter how small, is valuable and appreciated. Together, we're building a more secure future!

🛡️ **Securing the Future, One Contribution at a Time** 🛡️