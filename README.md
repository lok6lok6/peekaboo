# 🚀 Peekaboo - Decentralized Identity Manager

Peekaboo is a **decentralized identity management (DID) system** built with **Rust, Axum, and PostgreSQL**.  
It allows users to **create, store, and manage Decentralized Identifiers (DIDs).**

## 📌 Features
✅ **Create a DID** (`POST /create_did`)  
✅ **Fetch all DIDs** (`GET /get_dids`)  
🔜 **Retrieve a specific DID** (`GET /get_did/{id}`)  
🔜 **Update a DID** (`PUT /update_did/{id}`)  
🔜 **Delete a DID** (`DELETE /delete_did/{id}`)  
🔜 **Authentication (JWT/API Keys)**  

---

## 🛠 Tech Stack
- **Language:** Rust 🦀  
- **Framework:** Axum  
- **Database:** PostgreSQL  
- **ORM:** SQLx  
- **Authentication:** Coming soon...  

---

## 🚀 Setup Instructions
### **🔹 Prerequisites**
- Install **Rust & Cargo**: [Rust Installation](https://www.rust-lang.org/tools/install)
- Install **PostgreSQL**: [PostgreSQL Installation](https://www.postgresql.org/download/)

### **🔹 Clone the Repository**
```sh
git clone https://github.com/YOUR_GITHUB_USERNAME/peekaboo.git
cd peekaboo/backend

🔹 Set Up the Database

psql -U your_user -d postgres -c "CREATE DATABASE peekaboo;"
psql -U your_user -d peekaboo -c "
CREATE TABLE dids (
    id TEXT PRIMARY KEY,
    public_key TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);"

🔹 Create an .env File

Inside backend/, create a .env file:

DATABASE_URL=postgres://your_user:your_password@localhost:5432/peekaboo

🔹 Run the Project

cargo run

🔹 Test the API

Create a DID:

curl -X POST http://127.0.0.1:3000/create_did \
     -H "Content-Type: application/json" \
     -d '{"public_key": "0x123456789abcdef"}'

Fetch All DIDs:

curl -X GET http://127.0.0.1:3000/get_dids

🛠 Roadmap

✅ Phase 1: DID Creation & Retrieval
🔜 Phase 2: DID Updates & Deletion
🔜 Phase 3: Authentication & Security
🔜 Phase 4: Web UI for Managing DIDs


⸻

🎯 Contributing

Contributions are welcome! Feel free to fork the repo and submit pull requests.

⸻

📜 License

MIT License - Feel free to use and modify Peekaboo!

⸻
