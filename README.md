# Final Exercise from 100 Exercises To Learn Rust by Mainmatter

## Description

The final exercise in chapter 08 Futures, 8.8 Outro:

> Try writing an **asynchronous REST API** to expose the functionality of the ticket management system we built throughout the course.
>
> It should expose endpoints to:
>   - Create a ticket
>   - Retrieve ticket details
>   - Patch a ticket
>
> Use Rust's package registry, crates.io, to find the dependencies you need (if any) to build this system.

### Goal

Build a working REST API for the ticket management exercise. Keep it simple, do not overcomplicate.

### Stack

- Rust
- Tokio
- Axum
- SQLx
- PostgreSQL


### Features that you know are cool and have read but are waaaay to early to implement 

- Authentication
- Authorization
- Passwords / Argon2id
- Sessions / cookies
- HTTPS
- Public deployment
- Tailscale
- Production security
- Docker
- systemd

## Development setup

```text
PC
└── Axum + SQLx
        │
        │ LAN
        ▼
Debian server
└── PostgreSQL