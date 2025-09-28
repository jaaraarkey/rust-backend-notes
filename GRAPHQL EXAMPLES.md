# 🎮 GraphQL API Examples - Smart Notes

Complete collection of GraphQL queries and mutations for the Smart Notes API with JWT authentication.

## 📋 **Table of Contents**

- [🔓 Public Operations](#-public-operations)
- [🔐 Authentication Flow](#-authentication-flow)
- [👤 User Management](#-user-management)
- [📝 Note Operations](#-note-operations)
- [🔍 Search & Discovery](#-search--discovery)
- [🛡️ Error Handling Examples](#️-error-handling-examples)
- [🧪 Testing Scenarios](#-testing-scenarios)
- [💡 Advanced Examples](#-advanced-examples)

---

## 🔓 **Public Operations**
*No authentication required*

### **🌍 Hello World**
```graphql
query HelloWorld {
  hello
}
```

**Response (Unauthenticated):**
```json
{
  "data": {
    "hello": "Hello! Welcome to Smart Notes API - please authenticate to access personalized features."
  }
}
```

### **📚 Get All Notes (Public)**
```graphql
query PublicNotes {
  allNotes {
    id
    title
    content
    createdAt
    updatedAt
  }
}
```

---

## 🔐 **Authentication Flow**

### **1. Register New User**
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
      updatedAt
      isActive
    }
  }
}
```

**Success Response:**
```json
{
  "data": {
    "register": {
      "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
      "user": {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "email": "developer@smartnotes.com",
        "fullName": "Smart Developer",
        "createdAt": "2024-01-15T10:30:00Z",
        "updatedAt": "2024-01-15T10:30:00Z",
        "isActive": true
      }
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
      createdAt
      isActive
    }
  }
}
```

### **3. Login with Validation Error**
```graphql
mutation InvalidLogin {
  login(input: {
    email: "invalid-email"
    password: "123"
  }) {
    token
    user {
      id
      email
    }
  }
}
```

**Error Response:**
```json
{
  "errors": [
    {
      "message": "Validation failed: email: Invalid email format, password: Password must be at least 8 characters long"
    }
  ]
}
```

---

## 👤 **User Management**
*Requires JWT token in Authorization header*

### **Request Headers for Authenticated Operations:**
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Content-Type: application/json
```

### **🔍 Get Current User Profile**
```graphql
query MyProfile {
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

**Response:**
```json
{
  "data": {
    "me": {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "email": "developer@smartnotes.com",
      "fullName": "Smart Developer",
      "createdAt": "2024-01-15T10:30:00Z",
      "updatedAt": "2024-01-15T10:30:00Z",
      "isActive": true
    }
  }
}
```

### **👋 Authenticated Hello**
```graphql
query AuthenticatedHello {
  hello
}
```

**Response (With JWT token):**
```json
{
  "data": {
    "hello": "Hello developer@smartnotes.com! You're authenticated with Smart Notes API featuring PostgreSQL, JWT auth, and AI-powered features!"
  }
}
```

---

## 📝 **Note Operations**
*All note operations require authentication*

### **✨ Create Note with Smart Auto-Title**
```graphql
mutation CreateSmartNote {
  createNote(input: {
    content: "Today I learned about Rust's ownership model and how it prevents memory leaks without garbage collection. The borrow checker ensures memory safety at compile time, which is revolutionary for systems programming."
  }) {
    id
    title
    content
    createdAt
    updatedAt
  }
}
```

**Response:**
```json
{
  "data": {
    "createNote": {
      "id": "456e7890-e12c-34d5-a678-123456789abc",
      "title": "Today I learned about Rust's ownership model",
      "content": "Today I learned about Rust's ownership model and how it prevents memory leaks without garbage collection. The borrow checker ensures memory safety at compile time, which is revolutionary for systems programming.",
      "createdAt": "2024-01-15T14:25:00Z",
      "updatedAt": "2024-01-15T14:25:00Z"
    }
  }
}
```

### **📝 Create Note with Custom Title**
```graphql
mutation CreateCustomTitleNote {
  createNote(input: {
    title: "My Custom Title"
    content: "This note has a manually specified title instead of auto-generated one."
  }) {
    id
    title
    content
    createdAt
  }
}
```

### **📚 Get My Notes**
```graphql
query MyNotes {
  notes {
    id
    title
    content
    createdAt
    updatedAt
  }
}
```

### **📖 Get Specific Note**
```graphql
query GetNote {
  note(id: "456e7890-e12c-34d5-a678-123456789abc") {
    id
    title
    content
    createdAt
    updatedAt
  }
}
```

### **✏️ Update Note**
```graphql
mutation UpdateNote {
  updateNote(
    id: "456e7890-e12c-34d5-a678-123456789abc"
    input: {
      title: "Updated: Rust Ownership Deep Dive"
      content: "Today I learned about Rust's ownership model and how it prevents memory leaks without garbage collection. The borrow checker ensures memory safety at compile time, which is revolutionary for systems programming. UPDATE: I also learned about lifetimes and how they work with the borrow checker."
    }
  ) {
    id
    title
    content
    updatedAt
  }
}
```

### **🗑️ Delete Note**
```graphql
mutation DeleteNote {
  deleteNote(id: "456e7890-e12c-34d5-a678-123456789abc")
}
```

**Response:**
```json
{
  "data": {
    "deleteNote": true
  }
}
```

---

## 🔍 **Search & Discovery**

### **🔎 Full-Text Search**
```graphql
query SearchNotes {
  searchNotes(query: "Rust ownership") {
    id
    title
    content
    createdAt
  }
}
```

### **🔍 Search Programming Notes**
```graphql
query SearchProgramming {
  searchNotes(query: "programming algorithm") {
    id
    title
    content
    createdAt
  }
}
```

### **📋 Search Meeting Notes**
```graphql
query SearchMeetings {
  searchNotes(query: "meeting discuss") {
    id
    title
    content
    createdAt
  }
}
```

---

## 🛡️ **Error Handling Examples**

### **❌ Unauthorized Access (No Token)**
```graphql
query UnauthorizedNotes {
  notes {
    id
    title
  }
}
```

**Error Response:**
```json
{
  "errors": [
    {
      "message": "Unauthorized"
    }
  ]
}
```

### **❌ Invalid Token**
```
Headers: Authorization: Bearer invalid_token_here
```
```graphql
query InvalidToken {
  me {
    id
    email
  }
}
```

### **❌ Empty Content Validation**
```graphql
mutation EmptyContent {
  createNote(input: {
    content: ""
  }) {
    id
    title
  }
}
```

**Error Response:**
```json
{
  "errors": [
    {
      "message": "Content cannot be empty"
    }
  ]
}
```

### **❌ Note Not Found**
```graphql
query NonexistentNote {
  note(id: "00000000-0000-0000-0000-000000000000") {
    id
    title
  }
}
```

### **❌ Duplicate Email Registration**
```graphql
mutation DuplicateEmail {
  register(input: {
    email: "developer@smartnotes.com"  # Already exists
    password: "password123"
    fullName: "Another User"
  }) {
    token
    user {
      id
    }
  }
}
```

---

## 🧪 **Testing Scenarios**

### **📋 Complete User Journey**
```graphql
# 1. Register
mutation Step1_Register {
  register(input: {
    email: "testuser@example.com"
    password: "testpass123"
    fullName: "Test User"
  }) {
    token
    user {
      id
      email
    }
  }
}

# 2. Create first note (use token from step 1)
mutation Step2_FirstNote {
  createNote(input: {
    content: "This is my first note in the Smart Notes API. The system will automatically generate a title for this content."
  }) {
    id
    title
    content
  }
}

# 3. Create second note
mutation Step3_SecondNote {
  createNote(input: {
    content: "Meeting notes from today's standup: Discussed the new features, reviewed the roadmap, and planned the next sprint."
  }) {
    id
    title
    content
  }
}

# 4. Get all my notes
query Step4_MyNotes {
  notes {
    id
    title
    content
    createdAt
  }
}

# 5. Search my notes
query Step5_SearchNotes {
  searchNotes(query: "meeting standup") {
    id
    title
    content
  }
}
```

### **🔄 Note Lifecycle Test**
```graphql
# Create
mutation CreateLifecycleNote {
  createNote(input: {
    content: "This note will be created, updated, and then deleted to test the complete lifecycle."
  }) {
    id
    title
    content
    createdAt
  }
}

# Update (use ID from create response)
mutation UpdateLifecycleNote {
  updateNote(
    id: "REPLACE_WITH_ACTUAL_ID"
    input: {
      content: "This note has been updated with new content to test the update functionality."
    }
  ) {
    id
    title
    content
    updatedAt
  }
}

# Delete
mutation DeleteLifecycleNote {
  deleteNote(id: "REPLACE_WITH_ACTUAL_ID")
}
```

---

## 💡 **Advanced Examples**

### **🎯 Smart Title Generation Showcase**

#### **Example 1: Long Content with Sentence**
```graphql
mutation SmartTitle1 {
  createNote(input: {
    content: "The quick brown fox jumps over the lazy dog. This is a complete sentence that should be preserved in the title generation. Additional content follows that won't be included in the auto-generated title."
  }) {
    title  # Expected: "The quick brown fox jumps over the lazy dog."
    content
  }
}
```

#### **Example 2: Question as Title**
```graphql
mutation SmartTitle2 {
  createNote(input: {
    content: "How does JWT authentication work in modern web applications? JWT (JSON Web Tokens) are a compact way to represent claims between two parties..."
  }) {
    title  # Expected: "How does JWT authentication work in modern web..."
    content
  }
}
```

#### **Example 3: List Content**
```graphql
mutation SmartTitle3 {
  createNote(input: {
    content: "Shopping list: milk, eggs, bread, cheese, butter, yogurt, bananas, apples"
  }) {
    title  # Expected: "Shopping list: milk, eggs, bread, cheese, butter"
    content
  }
}
```

#### **Example 4: Code Snippet**
```graphql
mutation SmartTitle4 {
  createNote(input: {
    content: "Rust function to calculate fibonacci numbers recursively: fn fibonacci(n: u32) -> u32 { match n { 0 => 0, 1 => 1, _ => fibonacci(n-1) + fibonacci(n-2) } }"
  }) {
    title  # Expected: "Rust function to calculate fibonacci numbers..."
    content
  }
}
```

### **🔍 Advanced Search Patterns**

#### **Multi-word Search**
```graphql
query AdvancedSearch1 {
  searchNotes(query: "rust programming language") {
    id
    title
    content
  }
}
```

#### **Technical Terms Search**
```graphql
query AdvancedSearch2 {
  searchNotes(query: "JWT authentication token") {
    id
    title
    content
  }
}
```

#### **Partial Word Matching**
```graphql
query AdvancedSearch3 {
  searchNotes(query: "meet") {  # Should match "meeting", "meetings"
    id
    title
    content
  }
}
```

### **📊 Bulk Operations Example**
```graphql
# Create multiple notes in sequence
mutation BulkCreate1 {
  note1: createNote(input: {
    content: "First bulk note: Project planning and initial setup"
  }) {
    id
    title
  }
}

mutation BulkCreate2 {
  note2: createNote(input: {
    content: "Second bulk note: Database schema design and implementation"
  }) {
    id
    title
  }
}

mutation BulkCreate3 {
  note3: createNote(input: {
    content: "Third bulk note: API development and testing strategies"
  }) {
    id
    title
  }
}
```

---

## 🎮 **GraphiQL Usage Tips**

### **📋 Setting Headers in GraphiQL**
```json
{
  "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### **🔧 Variables Example**
```graphql
mutation CreateNoteWithVariables($content: String!) {
  createNote(input: {
    content: $content
  }) {
    id
    title
    content
  }
}
```

**Query Variables:**
```json
{
  "content": "This note content comes from a GraphQL variable for better reusability."
}
```

### **📝 Multi-operation Document**
```graphql
query GetProfile {
  me {
    id
    email
    fullName
  }
}

query GetNotes {
  notes {
    id
    title
    content
  }
}

mutation CreateNote($content: String!) {
  createNote(input: {
    content: $content
  }) {
    id
    title
  }
}
```

---

## 🚀 **cURL Examples**

### **Register via cURL**
```bash
curl -X POST http://127.0.0.1:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation { register(input: { email: \"curl@test.com\", password: \"password123\", fullName: \"cURL User\" }) { token user { id email } } }"
  }'
```

### **Create Note via cURL**
```bash
curl -X POST http://127.0.0.1:8000/graphql \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "query": "mutation { createNote(input: { content: \"Note created via cURL command\" }) { id title content } }"
  }'
```

### **Search Notes via cURL**
```bash
curl -X POST http://127.0.0.1:8000/graphql \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "query": "query { searchNotes(query: \"cURL\") { id title content } }"
  }'
```

---

## 🎯 **Best Practices**

### **✅ Always Use Meaningful Queries**
```graphql
# Good: Specific query name and fields
query GetUserNotesForDashboard {
  notes {
    id
    title
    createdAt
  }
}

# Avoid: Generic query without context
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

### **🔒 Handle Authentication Properly**
```graphql
# Always check authentication status first
query CheckAuth {
  hello  # Will indicate if you're authenticated
}

# Then proceed with protected operations
query ProtectedOperation {
  me {
    id
    email
  }
  notes {
    id
    title
  }
}
```

### **⚡ Optimize Field Selection**
```graphql
# Good: Only request needed fields
query OptimizedNotes {
  notes {
    id
    title
    createdAt  # Only what you need
  }
}

# Avoid: Requesting all fields when not needed
query UnoptimizedNotes {
  notes {
    id
    title
    content      # Large field
    createdAt
    updatedAt
  }
}
```

---

**🎸🔥💙 Happy GraphQL querying with your Smart Notes API!**

*Test these examples at: http://127.0.0.1:8000/graphiql*