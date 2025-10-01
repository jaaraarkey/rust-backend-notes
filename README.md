# 🚀 Smart Notes API - Complete GraphQL Backend

A sophisticated note-taking API built with **Rust**, **GraphQL**, **PostgreSQL**, and **JWT Authentication**. Features advanced folder organization, full-text search, and AI-powered auto-title generation.

## ✨ **Features**

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

### 📁 **Advanced Folder System**
- **Hierarchical folders** - Organize notes with parent/child relationships
- **Folder customization** - Colors, icons, and descriptions
- **Smart organization** - Default folders for new users
- **Folder statistics** - Note counts and activity tracking
- **Drag & drop support** - Move notes between folders seamlessly

### 🎯 **Smart Note Management**
- **Intelligent auto-title generation** from content analysis
- **Pinned notes** - Mark important notes for quick access
- **Full-text search** powered by PostgreSQL
- **User-specific notes** with proper isolation
- **Content validation** and sanitization
- **Timestamp tracking** for creation and updates
- **Word count & analytics** - Automatic content analysis

### 🌐 **Modern API Design**
- **GraphQL API** with async-graphql for type-safe operations
- **Interactive GraphiQL** playground for testing
- **Beautiful landing page** with feature showcase
- **Comprehensive error handling** with categorized responses
- **CORS support** for cross-origin requests

## 🏗️ **Architecture**

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   GraphQL API   │────│  Rust Backend    │────│   PostgreSQL    │
│   (Port 8000)   │    │  (Authentication)│    │   (Port 5433)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                        │
    ┌─────────┐             ┌──────────┐            ┌─────────────┐
    │GraphiQL │             │   JWT    │            │  Migrations │
    │Explorer │             │ Tokens   │            │   & Schema  │
    └─────────┘             └──────────┘            └─────────────┘
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
Add to your request headers in GraphiQL:
```json
{
  "Authorization": "Bearer YOUR_JWT_TOKEN_HERE"
}
```

## 📖 **GraphQL API Documentation**

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

#### **Login User**  
```graphql
mutation {
  login(input: {
    email: "user@example.com"
    password: "securepassword123"
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

---

### **📁 Folder Management**

#### **Create Folder**
```graphql
mutation {
  createFolder(input: {
    name: "Work Projects"
    description: "All my work-related notes and documentation"
    color: "#3B82F6"
    icon: "briefcase"
    position: 1
  }) {
    id
    name
    description
    color
    icon
    position
    isDefault
    createdAt
    updatedAt
  }
}
```

#### **Get All User Folders**
```graphql
query {
  folders {
    id
    name
    description
    color
    icon
    position
    notesCount
    isDefault
    createdAt
    updatedAt
  }
}
```

#### **Get Specific Folder**
```graphql
query {
  folder(id: "123e4567-e89b-12d3-a456-426614174000") {
    id
    name
    description
    color
    icon
    notesCount
    isDefault
    subfolders {
      id
      name
      color
    }
  }
}
```

#### **Update Folder**
```graphql
mutation {
  updateFolder(
    id: "123e4567-e89b-12d3-a456-426614174000"
    input: {
      name: "Updated Project Name"
      description: "New description"
      color: "#10B981"
      icon: "star"
      position: 2
    }
  ) {
    id
    name
    description
    color
    icon
    position
    updatedAt
  }
}
```

#### **Delete Folder**
```graphql
mutation {
  deleteFolder(id: "123e4567-e89b-12d3-a456-426614174000")
}
```

---

### **📝 Note Management**

#### **Create Note**
```graphql
mutation {
  createNote(input: {
    content: "This is my first Smart Note! It supports **markdown** formatting and automatic title generation."
    title: "My First Note"
    folderId: "123e4567-e89b-12d3-a456-426614174000"
    isPinned: false
  }) {
    id
    title
    content
    folderId
    isPinned
    wordCount
    viewCount
    createdAt
    updatedAt
    folder {
      id
      name
      color
    }
  }
}
```

#### **Create Note with Auto-Title**
```graphql
mutation {
  createNote(input: {
    content: "Meeting notes from today's sprint planning. Discussed new features for the Smart Notes API including folder hierarchies and advanced search capabilities."
  }) {
    id
    title  # Will be auto-generated: "Meeting notes from today's sprint planning..."
    content
    wordCount
    createdAt
  }
}
```

#### **Get All User Notes**
```graphql
query {
  notes {
    id
    title
    content
    isPinned
    pinnedAt
    wordCount
    viewCount
    createdAt
    updatedAt
    folder {
      id
      name
      color
      icon
    }
  }
}
```

#### **Update Note**
```graphql
mutation {
  updateNote(
    id: "note-uuid-here"
    input: {
      title: "Updated Note Title"
      content: "Updated content with new information and insights."
    }
  ) {
    id
    title
    content
    updatedAt
    wordCount
  }
}
```

#### **Delete Note**
```graphql
mutation {
  deleteNote(id: "note-uuid-here")
}
```

---

### **📌 Note Organization**

#### **Pin/Unpin Note**
```graphql
mutation {
  toggleNotePin(noteId: "note-uuid-here") {
    id
    title
    isPinned
    pinnedAt
  }
}
```

#### **Get Pinned Notes**
```graphql
query {
  pinnedNotes {
    id
    title
    content
    isPinned
    pinnedAt
    folder {
      name
      color
    }
  }
}
```

#### **Move Note to Folder**
```graphql
mutation {
  moveNoteToFolder(
    noteId: "note-uuid-here"
    input: {
      targetFolderId: "folder-uuid-here"
      position: 1
    }
  ) {
    id
    title
    folderId
    folder {
      id
      name
      color
    }
  }
}
```

#### **Get Notes in Folder**
```graphql
query {
  notesInFolder(folderId: "folder-uuid-here") {
    id
    title
    content
    isPinned
    wordCount
    createdAt
    folder {
      name
      color
    }
  }
}
```

---

### **🔍 Search & Discovery**

#### **Search Notes**
```graphql
query {
  searchNotes(query: "GraphQL API development") {
    id
    title
    content
    wordCount
    createdAt
    folder {
      name
      color
    }
  }
}
```

#### **Complex Search Examples**
```graphql
# Search for notes containing "rust" and "graphql"
query {
  searchNotes(query: "rust graphql") {
    id
    title
    # Highlights matching terms
    content
    folder { name }
  }
}

# Search in specific topics
query {
  searchNotes(query: "postgresql database optimization") {
    id
    title
    content
    wordCount
    viewCount
  }
}
```

---

### **🎯 Advanced Queries**

#### **Dashboard Overview**
```graphql
query {
  # Get user profile
  me {
    id
    email
    fullName
  }
  
  # Get folder structure
  folders {
    id
    name
    color
    notesCount
    isDefault
  }
  
  # Get recent notes
  notes {
    id
    title
    createdAt
    isPinned
    folder {
      name
      color
    }
  }
  
  # Get pinned notes
  pinnedNotes {
    id
    title
    pinnedAt
  }
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

### **Folders Table**
```sql
CREATE TABLE folders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    color TEXT NOT NULL DEFAULT '#3B82F6',
    icon TEXT NOT NULL DEFAULT 'folder',
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES folders(id) ON DELETE CASCADE,
    position INTEGER NOT NULL DEFAULT 0,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### **Notes Table**
```sql
CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    folder_id UUID REFERENCES folders(id) ON DELETE SET NULL,
    is_pinned BOOLEAN DEFAULT FALSE,
    pinned_at TIMESTAMPTZ,
    view_count INTEGER DEFAULT 0,
    word_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX notes_search_idx ON notes 
USING GIN (to_tsvector('english', title || ' ' || content));
CREATE INDEX notes_user_id_idx ON notes (user_id);
CREATE INDEX notes_folder_id_idx ON notes (folder_id);
CREATE INDEX folders_user_id_idx ON folders (user_id);
```

### **Key Relationships**
```sql
users (1) ────────── (∞) folders
folders (1) ────────── (∞) notes
folders (1) ────────── (∞) folders (hierarchy)
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
│   ├── types.rs             # GraphQL schema types & folder definitions
│   ├── web.rs               # Web handlers & GraphiQL interface
│   └── validation.rs        # Input validation & sanitization
├── migrations/              # Database migration files
│   ├── 20250928144430_create_notes_table.sql
│   ├── 20250928155448_create_users_table.sql
│   └── 20250930000001_folders_system.sql
├── Cargo.toml              # Rust dependencies & metadata
├── .env                    # Environment variables (create manually)
├── README.md               # This comprehensive documentation
├── GRAPHQL EXAMPLES.md     # Extended GraphQL query examples
├── CONTRIBUTING.md         # Contribution guidelines
├── CHANGELOG.md            # Version history and roadmap
└── LICENSE                 # MIT License
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

## 🎸🔥💙 **What You've Built**

Your Smart Notes API is now a **production-ready GraphQL backend** with:

✅ **Complete Folder System** - Hierarchical organization with parent/child relationships  
✅ **Advanced Note Features** - Pinning, word count, view tracking, folder assignment  
✅ **JWT Authentication** - Secure user registration, login, and session management  
✅ **PostgreSQL Database** - Full-featured with migrations and proper indexing  
✅ **GraphQL API** - Type-safe with interactive playground  
✅ **Smart Features** - Auto-title generation, full-text search capabilities  

## 🏆 **Production Readiness**

### **✅ Implemented**
- JWT authentication with middleware
- PostgreSQL with connection pooling
- Comprehensive error handling
- Input validation and sanitization
- Auto-generated API documentation
- CORS support for cross-origin requests
- Complete folder management system
- Advanced note organization features

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

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙋‍♂️ **Support**

- **Documentation**: This README and inline code comments
- **GraphiQL**: Interactive API explorer at `/graphiql`
- **Issues**: GitHub Issues for bug reports and feature requests

---

**🎸🔥💙 Built with Rust, PostgreSQL, GraphQL, and pure genius!**

*Smart Notes API - Where intelligent note-taking meets production-ready architecture with advanced folder management.*
