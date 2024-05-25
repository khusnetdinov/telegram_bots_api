# Telegram

![Static Badge](https://img.shields.io/badge/Project_Status-development-red)
[![codecov](https://codecov.io/gh/khusnetdinov/telegram_bots_api/graph/badge.svg?token=HODA8WDALK)](https://codecov.io/gh/khusnetdinov/telegram_bots_api)
[![crates.io](https://img.shields.io/crates/v/telegram_bots_api.svg)](https://crates.io/crates/telegram_bots_api)
![Static Badge](https://img.shields.io/badge/Telegram_Bot_API-7.3-green)
[![docs.rs](https://img.shields.io/docsrs/telegram_bots_api)](https://docs.rs/telegram_bots_api/latest/telegram_bots_api/)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fkhusnetdinov%2Ftelegram.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fkhusnetdinov%2Ftelegram?ref=badge_shield)

![https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/codecov_report/badge.svg](https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/codecov_report.yml/badge.svg)
![https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/lints/badge.svg](https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/lints.yml/badge.svg)
![https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/msrv/badge.svg](https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/msrv.yml/badge.svg)
![https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/features/badge.svg](https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/features.yml/badge.svg)
![https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/tests/badge.svg](https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/tests.yml/badge.svg)
![https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/rolling/badge.svg](https://github.com/khusnetdinov/telegram_bots_api/actions/workflows/rolling.yml/badge.svg)

  Telegram bots api simple rust wrapper, and no more.

### Features

  - async: asynchronous execution api calls
  - sync: synchronous execution api calls

### Installation

Run `cargo add telegram_bots_api`, or add lines to `Cargo.toml`:

```toml
[dependencies]
telegram_bots_api = "0.73.0"
rust-version = "1.70.0"
```

### Configuration

```

USAGE:
    sync-playground [FLAGS] [OPTIONS] --token <token>

FLAGS:
    -d, --debug         Environment: Debug mode
    -h, --help          Prints help information
    -p, --production    Environment: Is production
    -V, --version       Prints version information

OPTIONS:
        --connect-timeout <connect-timeout>    Client: Connect timeout in secs. Set a timeout for only the connect phase
                                               [default: 5]
        --timeout <timeout>                    Client: Timeout in secs. The timeout is applied from when the request
                                               starts connecting until the response body has finished [default: 5]
        --token <token>                        Telegram: Token
        --updates-limit <updates-limit>        Updates: Limits the number of updates to be retrieved [default: 100]
        --updates-offset <updates-offset>      Updates: Identifier of the first update to be returned [default: 0]
        --updates-timeout <updates-timeout>    Updates: Timeout in seconds for long polling [default: 0]
        --url <url>                            Telegram: Api url [default: https://api.telegram.org]
```

### How to use

Only one required flag is `token`, example run:

```
$ cargo run -- --token 0000000000:XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

### File structure:

```
.
├── Cargo.lock                                        # Lock file
├── Cargo.toml                                        # Toml file
├── LICENSE                                           # LICENSE
├── README.md                                         # README
├── /examples                                         # Examples
│   ├── /async
│   │   ├── get_me.rs
│   │   └── playground.rs
│   └── /sync
│       ├── get_me.rs
│       └── playground.rs
├── postman.json                                      # Postman collection
└── /src                                              # Source code
    ├── /api                                          # Api structs definitions
    │   ├── /enums                                    # Enums
    │   │   ├── bot_command_scope.rs
    │   │   ...
    │   │   └── reply_markup.rs
    │   ├── enums.rs
    │   ├── mod.rs
    │   ├── /params                                   # Params structs for payload
    │   │   ├── add_sticker_to_set.rs
    │   │   ...
    │   │   └── upload_sticker_file.rs
    │   ├── params.rs
    │   ├── /requests                                 # Request traits
    │   │   ├── async.rs
    │   │   └── sync.rs
    │   ├── requests.rs
    │   ├── /responses                                # Telegram responses structs definitions
    │   │   ├── error.rs
    │   │   ├── parameters.rs
    │   │   └── result.rs
    │   ├── responses.rs
    │   ├── /types                                    # Telegram types 
    │   │   ├── animation.rs
    │   │   ...
    │   │   └── write_access_allowed.rs
    │   └── types.rs
    ├── /clients                                      # Clients definitions
    │   ├── async.rs
    │   ├── mod.rs
    │   └── sync.rs
    ├── config.rs                                     # Config definition
    ├── errors.rs                                     # Errors definition
    ├── lib.rs
    └── /tests
        ├── /clients                                  # Cleints
        │   ├── async.rs                              # Async client tests
        │   ├── mod.rs
        │   └── sync.rs                               # Sync client tests
        ├── config.rs                                 # Config tests
        ├── errors.rs                                 # Errors tests
        ├── /helpers                                  # Helpers for tests
        │   ├── mocked_async.rs
        │   ├── mocked_sync.rs
        │   └── mod.rs
        ├── mod.rs
        └── responses                                 # Example of json responses
            ├── add_sticker_to_set_error.json
            ...
            └── upload_sticker_file_success.json
```