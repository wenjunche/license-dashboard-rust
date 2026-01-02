# OpenFin License Dashboard

A full-stack web application for managing OpenFin application configurations with Rust backend and SvelteKit frontend.

## Project Structure

```
license-dashboard-rust/
├── backend/           # Rust API server (Axum + PostgreSQL)
├── frontend/          # SvelteKit + TypeScript
└── docker-compose.yml # PostgreSQL database
```

## Prerequisites

- Rust 1.70+ and Cargo
- Node.js 18+ and npm
- Docker and Docker Compose
- PostgreSQL (or use Docker Compose)

## Quick Start

### 1. Start PostgreSQL

```bash
docker-compose up -d
```

### 2. Setup Backend

```bash
cd backend

# Install sqlx-cli for migrations
cargo install sqlx-cli --no-default-features --features postgres

# Copy environment file
cp .env.example .env

# Run migrations
sqlx migrate run

# Start server
cargo run
```

Backend will run on `http://localhost:3000`

### 3. Setup Frontend

```bash
cd frontend
npm install
npm run dev
```

Frontend will run on `http://localhost:5173`

## Features

- **Data Table** with filtering, sorting, and pagination
- **Multi-field Filters**: contract, URL, UUID, app name, billable status
- **Match Types**: Exact or fuzzy matching for text fields
- **Bulk Operations**: Edit multiple records at once
- **CSV Export**: Download filtered data
- **Google OAuth**: Secure authentication (TODO)
- **CRUD Operations**: Create, read, update, delete app configurations

## API Endpoints

- `GET /api/app-configs` - List with filters
- `POST /api/app-configs` - Create new
- `GET /api/app-configs/:id` - Get by ID
- `PUT /api/app-configs/:id` - Update
- `DELETE /api/app-configs/:id` - Delete
- `POST /api/app-configs/bulk-update` - Bulk update
- `GET /api/app-configs/export` - Export CSV
- `GET /auth/google` - Google OAuth login (TODO)
- `GET /auth/callback` - OAuth callback (TODO)

## Development

Backend auto-reloads with `cargo watch`:
```bash
cargo install cargo-watch
cd backend
cargo watch -x run
```

Frontend hot-reloads automatically with Vite.

## Deployment

See [DEPLOYMENT.md](DEPLOYMENT.md) for AWS deployment instructions.

The project includes a GitHub Actions workflow for automated deployment to AWS ECR.

## License

MIT
