# Telegram

https://github.com/khusnetdinov/telegram/actions/workflows/rust/badge.svg

### Project Status
- TODO

### Description
- TODO

### Features
- TODO

### Roadmap
- Implement Sync Api
- Implement Async Api
- Implement Client settings:
  - Long polling
  - WebHook listen Http
  - WebHook listen Https
  - Switching to api (api <=> local)
- Implement Async / Sync executions (Async response)
- Specs
- Examples
- Docker
- FFI

### Files structure
```
├── /.github/                   # Github settings
├── /src/                       # The source code
│   ├── /api/                   # Telegram Api relative structs, methods
│   │   ├── /requests/          # Requests traits
│   │   │   ├── async.rs        # Async requests trait
│   │   │   └── blocking.rs     # Sync requests trait
│   │   │
│   │   ├── mod.rs              # mod
│   │   ├── params.rs           # Telegram Api request params structs
│   │   ├── requests.rs         # mod
│   │   ├── responses.rs        # Telegram Api responses structs
│   │   └── types.rs            # Telegram Api types
│   │
│   ├── /clients/               # Not compiled Elm files
│   │   ├── mod.rs              # mod
│   │   ├── async.rs            # Async client
│   │   ├── blocking.rs         # Sync client
│   │   └── traits.rs           # Clients traits
│   │
│   ├── /tests/                 # Not compiled Elm files
│   │   ├── mod.rs              # mod file
│   │   ├── api.rs              # Api tests
│   │   ├── client.rs           # Main client tests
│   │   ├── clients.rs          # Clients tests
│   │   ├── config.rs           # Config tests
│   │   └── errors.rs           # Errors tests
│   │   
│   ├── lib.rs
│   ├── client.rs               # Client structure realization
│   ├── config.rs               # Configuration, ENV variables, default values
│   └── errors.rs               # Errors enum with realization
│
│── .clippy.toml                # Clippy settings file
│── .gitignore                  # Git ignored files
│── .rustfmt.toml               # Rustfmt settings file
│── Cargo.lock                  # Cargo lock file
│── Cargo.toml                  # Cargo file
│── LICENSE                     # License
│── postman.json                # Postman collection for Telegram Api
└── README.md                   # README.md
```

### Installation

- TODO

### Configuration

- TODO

### Commands

- TODO

### How to use

- TODO

### Examples

- TODO
