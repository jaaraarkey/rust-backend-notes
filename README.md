# Notes App Backend 📝

A modern GraphQL API server built with Rust, featuring clean architecture and progressive complexity.

## 🚀 Quick Start

```bash
# Clone and run
git clone <your-repo>
cd backend
cargo run

# Visit GraphiQL playground
open http://127.0.0.1:8000
```

## 🏗️ Architecture

```text
src/
├── main.rs       # 🎯 Application entry point
├── types.rs      # 📝 GraphQL schema definitions  
├── resolvers.rs  # ⚡ Business logic & resolvers
├── data.rs       # 🗄️ Data access layer
└── web.rs        # 🌐 HTTP handlers & UI
```

## 📊 Current Features (Day 4)

- ✅ **GraphQL API** with full introspection
- ✅ **Interactive GraphiQL** playground  
- ✅ **UUID-based IDs** for global uniqueness
- ✅ **Type-safe schema** with Rust benefits
- ✅ **CRUD Operations**: Create notes (more coming Day 5)
- ✅ **Clean Architecture** with modular design

## 🎯 Learning Roadmap

| Day | Feature | Status |
|-----|---------|--------|
| 1-2 | GraphQL Foundation | ✅ Complete |
| 3 | UUID & Error Handling | ✅ Complete |
| 4 | Mutations & Clean Architecture | ✅ Complete |
| 5 | Update/Delete Operations | 🔄 Next |
| 6 | Validation & Polish | 📅 Planned |
| 7 | Flutter Integration | 📅 Planned |
| 8-14 | Database, Auth, Deployment | 📅 Planned |

## 🔧 GraphQL Schema

```graphql
type Query {
  hello: String!
  notes: [Note!]!
  note(id: String!): Note
}

type Mutation {
  createNote(input: CreateNoteInput!): Note!
}

type Note {
  id: String!      # UUID format
  title: String!
  content: String!
}
```

## 💻 Example Operations

### Create Note
```graphql
mutation {
  createNote(input: {
    title: "My New Note"
    content: "Created with GraphQL!"
  }) {
    id
    title
    content
  }
}
```

### List Notes
```graphql
query {
  notes {
    id
    title
    content
  }
}
```

### Get Single Note
```graphql
query {
  note(id: "uuid-here") {
    title
    content
  }
}
```

## 🛠️ Tech Stack

- **Language**: Rust 🦀
- **Web Framework**: Axum
- **GraphQL**: async-graphql
- **Runtime**: Tokio
- **IDs**: UUID v4

## 📚 Documentation

Run `cargo doc --open` to browse the full API documentation with examples and implementation details.

## 🎯 Next Steps

Ready for **Day 5**? We'll add:
- Update note mutations
- Delete note mutations  
- Complete CRUD operations
- Enhanced error handling

Happy coding! 🚀
