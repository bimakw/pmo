<div align="center">

# ⚔️ Percival

**Your knight in project management**

A modern, full-stack Project Management Office application built with Next.js, Rust, and PostgreSQL.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/bimakw/percival)](https://github.com/bimakw/percival/releases)
[![TypeScript](https://img.shields.io/badge/TypeScript-61.9%25-3178c6)](https://github.com/bimakw/percival)
[![Rust](https://img.shields.io/badge/Rust-34.0%25-dea584)](https://github.com/bimakw/percival)

**[Live Demo](https://percival-pmo.netlify.app)**

</div>

---

## ✨ Features

- **Project Management** — Create, update, and track projects with timelines and budgets
- **Task Management** — Assign tasks, track progress, set priorities and due dates
- **Team & Resources** — Manage teams, assign members, track resource allocation
- **Dashboard & Reports** — Visual analytics, progress charts, and reporting
- **Authentication** — Secure user registration and login system

---

## 🛠 Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Next.js 16 + TypeScript + Tailwind CSS |
| Backend | Rust + Axum |
| Database | PostgreSQL |
| DevOps | Docker + Docker Compose |

---

## 📁 Project Structure

```
percival/
├── frontend/           # Next.js TypeScript frontend
├── backend/            # Rust Axum API server
├── database/           # PostgreSQL schema and migrations
└── docs/               # Documentation
```

---

## 🚀 Quick Start

### Using Docker (Recommended)

```bash
# Clone the repository
git clone https://github.com/bimakw/percival.git
cd percival

# Build and run all services
docker compose up -d

# Or use make
make build
make run
```

### Services

| Service | URL |
|---------|-----|
| Frontend | http://localhost:3000 |
| Backend API | http://localhost:8080 |
| PostgreSQL | localhost:5432 |

---

## 💻 Development Setup

### Prerequisites

- Node.js 18+
- Rust 1.75+
- Docker & Docker Compose

### Run Development Environment

```bash
# Start database only
make dev-db

# Run backend (in separate terminal)
make dev-backend

# Run frontend (in separate terminal)
make dev-frontend
```

### Manual Database Setup

```bash
# Create database
createdb percival_db

# Run schema
psql -d percival_db -f database/schema.sql

# Load seed data (optional)
psql -d percival_db -f database/seed.sql
```

---

## 📡 API Endpoints

### Authentication
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/auth/register` | Register new user |
| POST | `/api/v1/auth/login` | Login |

### Projects
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/projects` | List all projects |
| POST | `/api/v1/projects` | Create project |
| GET | `/api/v1/projects/{id}` | Get project details |
| PUT | `/api/v1/projects/{id}` | Update project |
| DELETE | `/api/v1/projects/{id}` | Delete project |
| GET | `/api/v1/projects/{id}/tasks` | Get project tasks |
| GET | `/api/v1/projects/{id}/milestones` | Get project milestones |

### Tasks
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/tasks` | List all tasks |
| POST | `/api/v1/tasks` | Create task |
| GET | `/api/v1/tasks/{id}` | Get task details |
| PUT | `/api/v1/tasks/{id}` | Update task |
| DELETE | `/api/v1/tasks/{id}` | Delete task |

### Teams
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/teams` | List all teams |
| POST | `/api/v1/teams` | Create team |
| GET | `/api/v1/teams/{id}` | Get team details |
| PUT | `/api/v1/teams/{id}` | Update team |
| DELETE | `/api/v1/teams/{id}` | Delete team |
| GET | `/api/v1/teams/{id}/members` | Get team members |
| POST | `/api/v1/teams/{id}/members` | Add team member |

---

## 🗄 Database Schema

| Table | Description |
|-------|-------------|
| `users` | User accounts with roles (admin, manager, member) |
| `teams` | Team groups |
| `team_members` | Team membership |
| `projects` | Project details with status and budget |
| `project_members` | Project membership |
| `milestones` | Project milestones |
| `tasks` | Task items with assignments |
| `task_comments` | Task comments/discussions |
| `activity_logs` | Audit trail |

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

Made with ☕ by [bimakw](https://github.com/bimakw)

</div>
