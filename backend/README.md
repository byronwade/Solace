# Solace Browser Backend

The Rust backend for the Solace browser, providing multi-engine support and security features.

## Prerequisites

- Rust (latest stable)
- PowerShell
- Visual Studio Build Tools with C++ support
- CMake

## Setup

1. First, set up CEF by running the setup script:

```powershell
.\scripts\setup_cef.ps1
```

2. Build the project:

```bash
cargo build
```

## Development

The backend consists of several key components:

- `engine/`: Browser engine implementations
  - `blink.rs`: Chromium (CEF) implementation
  - More engines to come (Gecko, WebKit, Servo)
- `ipc/`: Communication with the frontend
- `network/`: Network routing (Direct, Tor, VPN)
- `security/`: Security features and sandboxing
- `ui/`: Web server for frontend communication

## Running

Start the backend server:

```bash
cargo run
```

The server will listen on `localhost:3001` for frontend connections.

## Architecture

The backend uses a modular architecture:

1. **Engine Layer**: Abstracts different browser engines behind a common interface
2. **IPC Layer**: Handles communication with the Next.js frontend
3. **Network Layer**: Manages different network routing strategies
4. **Security Layer**: Implements sandboxing and security features

## Adding a New Engine

1. Create a new module in `engine/`
2. Implement the `Engine` trait
3. Add engine type to `BrowserEngine` enum
4. Update `EngineManager::switch_engine`

## Security Considerations

- All engine processes are sandboxed
- Network traffic is isolated per engine
- IPC is restricted to localhost
- Certificate verification is enforced 