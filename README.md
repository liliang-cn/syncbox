SyncBox (prototype)

This repository contains a Rust backend scaffolded with TDD and DDD principles and a minimal Axum HTTP service. It also includes a comprehensive architecture document for the future SyncBox product.

Highlights

- Rust + Axum web service with clean layering (domain, application, infrastructure, presentation)
- Tests for domain, application use cases, and HTTP endpoints
- In-memory repository for rapid iteration
- Design document: docs/SYNCBOX_DESIGN.md

Getting started

- Run tests:
  cargo test

- Run the server:
  cargo run

The server will start on http://127.0.0.1:3000 and exposes:
- GET /health
- POST /todos

Project structure

- src/domain: Entities, value objects, domain errors, repository traits
- src/application: Use cases (application services)
- src/infrastructure: Adapters (in-memory repository)
- src/presentation: HTTP (Axum) routing and handlers

License

MIT (TBD)
