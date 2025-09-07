# 🦀 Rust Backend Notes API

A simple learning project: REST API for notes built with **Rust** using [Axum](https://github.com/tokio-rs/axum).

## 🚀 Features (implemented so far)
- `GET /hello` — returns a test JSON response
- `GET /note/example` — returns an example note
- (WIP) `POST /note` — create a new note
- (WIP) `GET /notes` — list all notes

## ⚙️ Installation & Run

### 1. Clone the repository
```bash
git clone git@github.com:<your_username>/rust-backend-notes.git
cd rust-backend-notes
```

### 2. Install Rust (if not installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Run the server
```bash
cargo run
```

The server will be available at:
```
http://127.0.0.1:3000
```

## 📡 API Examples

### Health check
```bash
curl http://127.0.0.1:3000/hello
```
Response:
```json
{"text":"Hello from JSON"}
```

### Example note
```bash
curl http://127.0.0.1:3000/note/example
```
Response:
```json
{
  "id": 1,
  "title": "First note",
  "content": "This is the content of the note"
}
```

## 🛠 Technologies
- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [Tokio](https://tokio.rs/)
- [Serde](https://serde.rs/)

## 🗺 Roadmap
- [ ] In-memory note storage (`Vec<Note>`)
- [ ] CRUD operations (`POST`, `GET`, `DELETE`)
- [ ] Database integration (SQLite / Postgres)
- [ ] User authentication (JWT)
- [ ] Connect Flutter frontend
- [ ] Deploy API (Railway, Fly.io, or Docker)

---

✍️ Author: [jaaraarkey](https://github.com/jaaraarkey)
