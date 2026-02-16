# Async Driver Management API (Axum + Tokio)

A simple HTTP server built using **Axum** and **Tokio** that manages F1 drivers.

This project demonstrates:

- Async Rust
- Multi-thread Tokio runtime
- Shared state using `Arc<RwLock<T>>`
- CRUD APIs

## 🧠 Architecture Overview

- Shared state: `Arc<RwLock<Vec<Driver>>>`
- Multiple readers allowed (GET requests)
- Single writer allowed (POST/PUT/DELETE)
- JSON file used as persistence layer
- Multi-thread Tokio runtime (default)

---

## ⚙️ Tech Stack

- Rust
- Axum
- Tokio (multi-thread runtime)
- Serde
- UUID (if used earlier)
- JSON file storage

---

## 📦 Driver Model

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Driver {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub team: String,
    pub points: u32,
}
IDs are generated in human-readable format:

DRV-001
DRV-002
DRV-003
 ```

## ▶️ Running the Server
cargo run
Server runs at:
http://127.0.0.1:4500

 API Endpoints
- Get All Drivers-:
curl -X GET http://127.0.0.1:4500/drivers
- Get Driver by ID-:
curl -X GET http://127.0.0.1:4500/drivers/DRV-001
- Add Driver-:
curl -X POST http://127.0.0.1:4500/drivers\
-H "Content-Type: application/json"\
-d '{
  "name": "Charles Leclerc",
  "team": "Ferrari",
  "points": 320
}'
- Update Driver
curl -X PUT http://127.0.0.1:4500/drivers/DRV-001 \
-H "Content-Type: application/json" \
-d '{
  "id": "",
  "name": "Max Verstappen",
  "team": "Red Bull Racing",
  "points": 600
}





