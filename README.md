# 🚀 Smart Notes GraphQL API with JWT Authentication

A production-ready GraphQL API built with Rust, featuring JWT authentication, PostgreSQL database, and intelligent note management with auto-title generation.

## ✨ Features

### 🔐 **Authentication & Security**
- **JWT-based authentication** with 24-hour token validity
- **bcrypt password hashing** with salt for secure storage
- **JWT middleware** for automatic route protection
- **User-specific data isolation** and access control
- **Bearer token authentication** following industry standards

### 🗄️ **Database & Persistence**
- **PostgreSQL integration** with SQLx for type-safe queries
- **Database migrations** with automatic schema management
- **Connection pooling** for optimal performance
- **ACID compliance** for data integrity

### 🎯 **Smart Note Management**
- **Intelligent auto-title generation** from content analysis
- **Full-text search** powered by PostgreSQL
- **User-specific notes** with proper isolation
- **Content validation** and sanitization
- **Timestamp tracking** for creation and updates

### 🌐 **Modern API Design**
- **GraphQL API** with async-graphql for type-safe operations
- **Interactive GraphiQL** playground for testing
- **Beautiful landing page** with feature showcase
- **Comprehensive error handling** with categorized responses
- **CORS support** for cross-origin requests

## 🏗️ **Architecture**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   GraphQL API   │────│ JWT Middleware  │────│  PostgreSQL DB  │
│  (async-graphql)│    │   (Auth Layer)  │    │   (SQLx Pool)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ├── Query Resolvers     ├── Token Verification  ├── User Management
         ├── Mutation Resolvers  ├── User Context       ├── Note Storage
         ├── Schema Generation   ├── Route Protection   ├── Full-text Search
         └── Error Handling      └── Auth Context       └── Migrations
```

## 🚦 **Quick Start**

### **Prerequisites**
- Rust 1.70+ installed
- Docker and Docker Compose
- PostgreSQL (via Docker)

### **1. Clone & Setup**
```bash
git clone <your-repo>
cd backend
```

### **2. Start PostgreSQL Database**
```bash
# Start PostgreSQL with Docker
docker run --name smart_notes_db \
  -e POSTGRES_PASSWORD=smartnotes2024 \
  -e POSTGRES_DB=smart_notes \
  -p 5433:5432 \
  -d postgres:15
```

### **3. Set Environment Variables**
```bash
# Create .env file
echo 'DATABASE_URL="postgresql://postgres:smartnotes2024@localhost:5433/smart_notes"' > .env
echo 'JWT_SECRET="your-super-secret-jwt-key-change-in-production"' >> .env
echo 'PORT=8000' >> .env
```

### **4. Build & Run**
```bash
# Install dependencies and build
cargo build --release

# Run database migrations and start server
cargo run
```

### **5. Access Your API**
- 🌟 **Landing Page**: http://127.0.0.1:8000
- 🎮 **GraphiQL Playground**: http://127.0.0.1:8000/graphiql
- 📡 **GraphQL Endpoint**: http://127.0.0.1:8000/graphql

## 🔐 **Authentication Flow**

### **1. Register a New User**
```graphql
mutation RegisterUser {
  register(input: {
    email: "developer@smartnotes.com"
    password: "supersecure123"
    fullName: "Smart Developer"
  }) {
    token
    user {
      id
      email
      fullName
      createdAt
      isActive
    }
  }
}
```

### **2. Login Existing User**
```graphql
mutation LoginUser {
  login(input: {
    email: "developer@smartnotes.com"
    password: "supersecure123"
  }) {
    token
    user {
      id
      email
      fullName
    }
  }
}
```

### **3. Use JWT Token for Authenticated Requests**
Add to your request headers:
```
Authorization: Bearer YOUR_JWT_TOKEN_HERE
```

## 📝 **API Examples**

### **🔓 Public Operations (No Auth Required)**

#### **Hello World**
```graphql
query {
  hello
}
```

#### **Register New Account**
```graphql
mutation {
  register(input: {
    email: "newuser@example.com"
    password: "securepassword123"
    fullName: "New User"
  }) {
    token
    user { id email fullName }
  }
}
```

### **🔐 Authenticated Operations (JWT Token Required)**

#### **Get Current User Profile**
```graphql
query {
  me {
    id
    email
    fullName
    createdAt
    updatedAt
    isActive
  }
}
```

#### **Create Smart Note**
```graphql
mutation {
  createNote(input: {
    content: "This is my note content. The system will automatically generate a smart title from this content using advanced text analysis algorithms."
  }) {
    id
    title          # Auto-generated intelligent title
    content
    createdAt
    updatedAt
  }
}
```

#### **Get User's Notes**
```graphql
query {
  notes {
    id
    title
    content
    createdAt
    updatedAt
  }
}
```

#### **Search Notes**
```graphql
query {
  searchNotes(query: "algorithm") {
    id
    title
    content
    createdAt
  }
}
```

#### **Update Note**
```graphql
mutation {
  updateNote(
    id: "note-uuid-here"
    input: {
      title: "Updated Title"
      content: "Updated content with new information."
    }
  ) {
    id
    title
    content
    updatedAt
  }
}
```

#### **Delete Note**
```graphql
mutation {
  deleteNote(id: "note-uuid-here")
}
```

## 🧠 **Smart Auto-Title Generation**

The API features intelligent title generation that:

- **Analyzes content semantics** to extract meaningful titles
- **Handles sentence boundaries** properly (respects punctuation)
- **Preserves quoted text integrity** (doesn't break on quotes)
- **Respects word boundaries** for clean truncation
- **Fallbacks gracefully** to "Untitled Note" for empty content
- **Limits to 50 characters** with smart truncation using "..."

### **Examples of Smart Titles**

| Content | Generated Title |
|---------|----------------|
| "Today I learned about Rust's ownership model. It's fascinating how..." | "Today I learned about Rust's ownership model" |
| "Meeting notes: Discussed the new feature roadmap. Key points include..." | "Meeting notes: Discussed the new feature..." |
| "Shopping list: milk, eggs, bread, cheese" | "Shopping list: milk, eggs, bread, cheese" |
| "The quick brown fox jumps over the lazy dog. This sentence contains..." | "The quick brown fox jumps over the lazy dog." |

## 🔍 **Full-Text Search**

Powered by PostgreSQL's advanced full-text search capabilities:

```sql
-- Example search query generated internally
SELECT * FROM notes 
WHERE to_tsvector('english', title || ' ' || content) 
@@ plainto_tsquery('english', $1)
ORDER BY ts_rank(to_tsvector('english', title || ' ' || content), 
                 plainto_tsquery('english', $1)) DESC;
```

## 🛡️ **Security Features**

### **Password Security**
- **bcrypt hashing** with automatic salt generation
- **Configurable cost factor** (default: 12)
- **Timing attack prevention** through constant-time comparison

### **JWT Security**
- **HMAC-SHA256 signing** with secret key
- **Configurable expiration** (default: 24 hours)
- **Automatic token validation** on every request
- **User context extraction** from valid tokens

### **Input Validation**
- **Email format validation** using regex patterns
- **Password strength requirements** (minimum 8 characters)
- **Content sanitization** to prevent injection attacks
- **UUID validation** for all ID parameters

## 📊 **Database Schema**

### **Users Table**
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    full_name TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);
```

### **Notes Table**
```sql
CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    user_id UUID REFERENCES users(id)
);

-- Full-text search index
CREATE INDEX notes_search_idx ON notes 
USING GIN (to_tsvector('english', title || ' ' || content));
```

## 🚀 **Performance Features**

### **Connection Pooling**
- **SQLx PgPool** for efficient database connections
- **Automatic connection management** and recycling
- **Configurable pool settings** for optimization

### **Async/Await Architecture**
- **Fully asynchronous** request handling with Tokio
- **Non-blocking I/O** for maximum throughput
- **Concurrent request processing** without thread blocking

### **Optimized Queries**
- **Prepared statements** for SQL injection prevention
- **Indexed searches** for fast full-text queries
- **Efficient pagination** support (ready for implementation)

## 🧪 **Testing Your API**

### **Using GraphiQL (Recommended)**
1. Visit http://127.0.0.1:8000/graphiql
2. Use the interactive playground to test queries
3. Built-in documentation and auto-completion

### **Using curl**
```bash
# Register user
curl -X POST http://127.0.0.1:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation { register(input: { email: \"test@example.com\", password: \"password123\", fullName: \"Test User\" }) { token user { id email } } }"
  }'

# Create authenticated note
curl -X POST http://127.0.0.1:8000/graphql \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "query": "mutation { createNote(input: { content: \"My test note content\" }) { id title content } }"
  }'
```

### **Using Postman**
Import the GraphQL schema and use Postman's GraphQL features for testing.

## 📁 **Project Structure**

```
backend/
├── src/
│   ├── main.rs              # Application entry point & JWT middleware
│   ├── auth.rs              # Authentication service & JWT handling
│   ├── database.rs          # PostgreSQL operations & migrations
│   ├── errors.rs            # Comprehensive error handling
│   ├── resolvers.rs         # GraphQL query/mutation resolvers
│   ├── types.rs             # GraphQL schema types
│   └── web.rs               # Web handlers & GraphiQL interface
├── migrations/              # Database migration files
├── Cargo.toml              # Rust dependencies & metadata
├── .env                    # Environment variables (create manually)
└── README.md               # This documentation
```

## ⚙️ **Configuration**

### **Environment Variables**
| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | Required | PostgreSQL connection string |
| `JWT_SECRET` | Auto-generated | Secret key for JWT signing |
| `PORT` | `8000` | Server port number |

### **JWT Configuration**
- **Token Validity**: 24 hours
- **Algorithm**: HMAC-SHA256
- **Refresh**: Manual (implement refresh tokens for production)

## 🏆 **Production Readiness**

### **✅ Implemented**
- JWT authentication with middleware
- PostgreSQL with connection pooling
- Comprehensive error handling
- Input validation and sanitization
- Auto-generated API documentation
- CORS support for cross-origin requests

### **🚧 Recommended for Production**
- [ ] Rate limiting middleware
- [ ] Request logging and monitoring
- [ ] Database backup strategy
- [ ] Health check endpoints
- [ ] Metrics collection (Prometheus)
- [ ] Environment-specific configurations
- [ ] TLS/SSL certificate setup
- [ ] Docker containerization
- [ ] Kubernetes deployment manifests

## 🤝 **Contributing**

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 **License**

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙋‍♂️ **Support**

- **Documentation**: This README and inline code comments
- **GraphiQL**: Interactive API explorer at `/graphiql`
- **Issues**: GitHub Issues for bug reports and feature requests

---

**🎸🔥💙 Built with Rust, PostgreSQL, GraphQL, and pure genius!**

*Smart Notes API - Where intelligent note-taking meets production-ready architecture.*
