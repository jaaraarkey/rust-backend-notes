# Notes App - Backend

A modern GraphQL API for a notes application built with Rust, Axum, and async-graphql.

## 🎯 Project Overview

This is the backend service for a full-stack notes application. The frontend is built with Flutter, and together they demonstrate modern async programming patterns and GraphQL best practices.

### Architecture
```
┌─────────────────┐    GraphQL     ┌─────────────────┐    SQLite    ┌─────────────┐
│   Flutter App   │ ──────────────▶│   Rust Backend  │ ───────────▶│  Database   │
│   (Frontend)    │                │     (Axum)      │              │             │
└─────────────────┘                └─────────────────┘              └─────────────┘
```

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (1.70+)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)
- [rust-analyzer](https://rust-analyzer.github.io/) (recommended for IDE support)

### Installation & Setup

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd backend
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Run the development server**
   ```bash
   cargo run
   ```

4. **Access GraphiQL Playground**
   Open your browser to: http://127.0.0.1:8000

## 📋 Development Roadmap

### Week 1: GraphQL Core (Current)
- [x] Day 1: Basic GraphQL setup with hello query
- [ ] Day 2: Note struct and static notes query
- [ ] Day 3: Single note query by ID
- [ ] Day 4: Create note mutation (in-memory)
- [ ] Day 5: Delete note mutation
- [ ] Day 6: Error handling
- [ ] Day 7: Code review and refactoring

### Week 2: Database Integration
- [ ] Day 8: SQLite integration
- [ ] Day 9: Database CRUD operations
- [ ] Day 10-14: Production deployment

## 🛠️ Technology Stack

| Category | Technology | Purpose |
|----------|------------|---------|
| **Language** | Rust | Systems programming language with memory safety |
| **Web Framework** | Axum | Modern, ergonomic web framework for Rust |
| **GraphQL** | async-graphql | GraphQL server implementation for Rust |
| **Database** | SQLite | Lightweight, serverless database |
| **Runtime** | Tokio | Asynchronous runtime for Rust |

## 📚 API Documentation

### Current Schema

```graphql
type Query {
  hello: String!
}

type Note {
  id: Int!
  title: String!
  content: String!
}
```

### Example Queries

**Hello Query**
```graphql
query {
  hello
}
```

Response:
```json
{
  "data": {
    "hello": "Hello from GraphQL!"
  }
}
```

## 🏗️ Project Structure

```
backend/
├── src/
│   ├── main.rs          # Application entry point
│   ├── schema/          # GraphQL schema definitions (future)
│   ├── resolvers/       # GraphQL resolvers (future)
│   └── models/          # Data models (future)
├── migrations/          # Database migrations (future)
├── Cargo.toml          # Rust dependencies
├── Cargo.lock          # Dependency lock file
├── README.md           # This file
└── .env.example        # Environment variables template (future)
```

## 🔧 Configuration

### Environment Variables

Create a `.env` file in the project root:

```bash
# Server Configuration
PORT=8000
HOST=127.0.0.1

# Database (coming in Week 2)
DATABASE_URL=sqlite:notes.db

# Logging
RUST_LOG=info
```

### Cargo.toml Dependencies

```toml
[dependencies]
axum = "0.7"                    # Web framework
tokio = { version = "1", features = ["full"] }  # Async runtime
async-graphql = "7.0"           # GraphQL implementation
async-graphql-axum = "7.0"      # Axum integration for GraphQL
serde = { version = "1", features = ["derive"] } # Serialization
```

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### GraphQL Testing

Use the GraphiQL playground at `http://127.0.0.1:8000` or use curl:

```bash
curl -X POST http://127.0.0.1:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ hello }"}'
```

## 🚀 Deployment

### Development
```bash
cargo run
```

### Production Build
```bash
cargo build --release
./target/release/backend
```

### Docker (Future)
```dockerfile
# Dockerfile will be added in Week 2
FROM rust:1.70 as builder
# ... build steps
```

## 🤝 Contributing

### Development Workflow

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Follow Rust naming conventions
   - Add tests for new functionality
   - Update documentation

3. **Test your changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

4. **Submit a pull request**

### Code Style

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## 🐛 Troubleshooting

### Common Issues

**Port already in use**
```bash
# Kill process on port 8000
lsof -ti:8000 | xargs kill -9
```

**Dependencies not found**
```bash
# Clean and rebuild
cargo clean
cargo build
```

**GraphiQL not loading**
- Ensure server is running on http://127.0.0.1:8000
- Check browser console for errors
- Verify CORS settings

## 📖 Learning Resources

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Axum Documentation](https://docs.rs/axum/)

### GraphQL
- [GraphQL Official Docs](https://graphql.org/learn/)
- [async-graphql Book](https://async-graphql.github.io/async-graphql/en/index.html)
- [GraphQL Best Practices](https://graphql.org/learn/best-practices/)

## 📝 Changelog

### Version 0.1.0 (Day 1)
- ✅ Initial project setup
- ✅ Basic GraphQL server with Axum
- ✅ Hello query implementation
- ✅ GraphiQL playground integration
- ✅ Development environment configuration

### Upcoming (Day 2)
- 📋 Note struct definition
- 📋 Static notes query
- 📋 Flutter project initialization

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👥 Authors

- ** Oleg Dyachenko aka jaar ** - *Initial work* - [jaar](https://github.com/jaaraarkey)

## 🙏 Acknowledgments

- Rust community for excellent async ecosystem
- async-graphql team for the GraphQL implementation
- Axum team for the modern web framework
