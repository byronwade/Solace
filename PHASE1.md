### High-Level Objectives for Phase 1

1.  **Rust Core Setup:**
    -   Create a new Rust binary project to orchestrate the browser.
    -   Set up logging, configuration, and a simple IPC layer scaffold.
2.  **CEF Integration (Blink Engine):**
    -   Download and integrate the Chromium Embedded Framework (CEF).
    -   Initialize CEF in Rust and create a basic browser window that can render a test page.
3.  **Next.js UI (using `app` directory):**
    -   Initialize a Next.js 13+ project with the `app` directory feature.
    -   Build a minimal UI for the browser shell (tabs, address bar, nav buttons).
    -   Compile the UI into static files to be served locally by Rust or loaded via `file://` or a local HTTP server.
4.  **Connecting UI and Engine via IPC:**
    -   Set up a mechanism for the Next.js UI to send commands (navigate, back, forward) to Rust.
    -   Rust forwards these commands to the CEF engine.
    -   Changes in the engine state (like navigating to a new URL) should update the UI if needed (initially just one-way commands from UI to engine is fine).
5.  **Minimal Security Measures and HTTPS Upgrades:**
    -   Implement a CSP for the Next.js UI.
    -   Force HTTPS upgrades where possible.
    -   Run CEF in a sandboxed mode (platform-dependent).

* * * * *

### Detailed Steps

#### Step 1: System Setup and Tooling

**Prerequisites:**

-   **Rust:**\
    Install using [rustup](https://rustup.rs/):

    bash

    Copy code

    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

-   **Node.js and npm:**\
    Use [nvm](https://github.com/nvm-sh/nvm) or your system's package manager:

    bash

    Copy code

    `nvm install --lts
    nvm use --lts`

-   **CEF Binaries:**\
    Visit the [CEF Builds site](https://cef-builds.spotifycdn.com/index.html) or the official CEF site and download a compatible binary for your platform. Extract it into `third_party/cef`.\
    For example:

    bash

    Copy code

    `mkdir -p third_party/cef
    # Extract cef_binary_...tar.gz or zip into this directory`

-   **CMake / Build Tools:**\
    Ensure `cmake` is installed (used for CEF integration):

    bash

    Copy code

    `# On Debian/Ubuntu
    sudo apt-get install cmake build-essential`

-   **Windowing and Rendering:**\
    On Linux, ensure `libgtk-3-dev`, `libnss3`, and other CEF prerequisites are installed. On Windows or macOS, ensure platform-specific requirements (Xcode command line tools on macOS, Visual Studio Build Tools on Windows).

#### Step 2: Rust Project Setup

Create a new directory for the Solace Browser project:

bash

Copy code

`mkdir solace_browser
cd solace_browser
cargo new --bin solace_core`

Your structure might look like this:

css

Copy code

`solace_browser/
├─ Cargo.toml
├─ src/
│  └─ main.rs
├─ ui/
│  └─ (Next.js project goes here)
└─ third_party/
   └─ cef/`

In `Cargo.toml`, add dependencies for logging, async runtime, and possibly IPC:

toml

Copy code

`[package]
name = "solace_browser"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"`

*(You'll add a `cef` crate or bindings once you have them. If no official `cef` crate exists, you may write Rust FFI bindings or use a third-party crate. For this walkthrough, we assume a hypothetical `cef-rs` crate.)*

#### Step 3: Integrate CEF with Rust

**Note:** The exact steps depend on the chosen CEF crate or integration strategy. The following is a conceptual approach:

-   Add a `[build-dependencies]` section for CEF if needed, or point your `build.rs` to find CEF headers and libraries.
-   In `src/main.rs`, initialize CEF:

rust

Copy code

`fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    // Hypothetical CEF init call:
    cef::initialize("third_party/cef")?;

    // Create a browser instance pointing to a placeholder page
    let browser = cef::Browser::new("SolaceBrowser", "about:blank")?;

    // Run the CEF event loop blocking call
    browser.run();

    Ok(())
}`

*(Adjust according to the actual API of your CEF integration crate.)*

At this point, you should be able to run `cargo run` and get a minimal CEF window displaying `about:blank`.

#### Step 4: Creating the Next.js (App Directory) UI

Initialize Next.js 13+ with the `app` directory:

bash

Copy code

`cd ui
npx create-next-app@latest --experimental-app
# When prompted:
# - Use TypeScript? yes (recommended)
# - Use Tailwind? yes
# - Use src directory? no
# - Use App Router? yes (default now)
# - ESLint? yes
# - etc.`

This scaffolds a project with `app/` directory-based routing. Your structure might be:

arduino

Copy code

`ui/
├─ app/
│  ├─ layout.tsx
│  └─ page.tsx
├─ public/
├─ package.json
├─ postcss.config.js
├─ tailwind.config.js
├─ tsconfig.json
└─ ...`

Install any needed components for your UI. For a minimal browser UI (address bar, back/forward buttons), you can use basic HTML elements styled with Tailwind, or integrate a UI component library.

Let's add a simple top-level UI:

-   `app/layout.tsx`:\
    Set up a global layout with a top bar (title bar) and a content region.

tsx

Copy code

`// ui/app/layout.tsx
import './globals.css'
import { Inter } from 'next/font/google'

const inter = Inter({ subsets: ['latin'] })

export const metadata = {
  title: 'Solace Browser',
  description: 'Phase 1 UI',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className={inter.className + " w-screen h-screen overflow-hidden bg-gray-100"}>
        {children}
      </body>
    </html>
  )
}`

-   `app/page.tsx`:\
    Add a simple UI with navigation buttons and an input for the URL:

tsx

Copy code

`// ui/app/page.tsx
'use client'
import { useState } from 'react'

export default function Page() {
  const [url, setUrl] = useState('https://example.com');

  const navigate = () => {
    // In Phase 1, we'll just print; later we'll send an IPC message to Rust.
    window.postMessage({ command: 'navigate', url }, '*');
  }

  const goBack = () => window.postMessage({ command: 'goBack' }, '*');
  const goForward = () => window.postMessage({ command: 'goForward' }, '*');
  const refresh = () => window.postMessage({ command: 'refresh' }, '*');

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center bg-gray-200 p-2">
        <button className="mr-2" onClick={goBack}>←</button>
        <button className="mr-2" onClick={goForward}>→</button>
        <button className="mr-2" onClick={refresh}>⟳</button>
        <input
          className="flex-grow border px-2 py-1 mr-2"
          value={url}
          onChange={e => setUrl(e.target.value)}
          onKeyDown={e => e.key === 'Enter' && navigate()}
        />
        <button className="mr-2" onClick={navigate}>Go</button>
      </div>
      <div className="flex-grow bg-white" id="browser-viewport">
        {/* The engine-rendered content will appear in a separate native area, not inside this DOM. */}
      </div>
    </div>
  )
}`

**Note:** `window.postMessage` here is a placeholder. You'll replace this with a more secure IPC channel. Eventually, you might inject a custom script that provides a `window.ipc` object, or use CEF's native messaging bridge.

#### Step 5: Build the Next.js UI for Production

bash

Copy code

`cd ui
npm run build
npm run export
# The exported static files will be in ui/out by default for static export`

You now have static files that can be served locally. If you rely on `next export`, ensure your page doesn't use dynamic server-side features unsupported by static export. Alternatively, run `npm run build` and `npm run start` to serve locally via Next.js. For Phase 1, static export is often simpler.

#### Step 6: Serving the UI Locally via Rust

Back in `solace_browser/`, integrate a simple HTTP server to serve the UI's static files:

Add `axum` and `tower-http` to `Cargo.toml`:

toml

Copy code

`[dependencies]
axum = "0.6"
tower-http = "0.3"`

Then in `src/main.rs`:

rust

Copy code

`use axum::{
    Router,
    routing::get,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    cef::initialize("third_party/cef")?;

    let app = Router::new().fallback_service(ServeDir::new("ui/out"));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let browser = cef::Browser::new("SolaceBrowser", "http://127.0.0.1:8080")?;
    browser.run();

    Ok(())
}`

Now `cargo run` should load the Next.js UI served by Axum. You'll see a navigation bar in the UI. When you click "Go," it posts a message to `window`.

#### Step 7: IPC Integration Between UI and Rust

To connect the UI's `postMessage` calls to Rust, you'll need a CEF mechanism for intercepting JavaScript calls. For example, using CEF's `cef::V8Handler` or a custom IPC bridge:

-   Inject a JavaScript object `window.ipc` in the renderer process that calls a native handler when `postMessage` is called. The details depend heavily on your chosen CEF bindings.
-   On receiving a `navigate` command, Rust calls `browser.load_url(url)`.
-   On `goBack`, `browser.go_back()` etc.

This might look like (pseudocode in Rust):

rust

Copy code

`// ipc.rs
pub fn handle_message(command: &str, payload: serde_json::Value) {
    match command {
        "navigate" => {
            if let Some(url) = payload["url"].as_str() {
                browser.load_url(url);
            }
        }
        "goBack" => browser.go_back(),
        "goForward" => browser.go_forward(),
        "refresh" => browser.reload(),
        _ => {}
    }
}`

Integrate this handler into the CEF message loop. For Phase 1, even a simple solution like setting a JavaScript binding (if the CEF crate allows it) or evaluating JS in response to load events might suffice. The main goal is to establish a pipeline for commands.

#### Step 8: Minimal Security Measures

-   **CSP:** Add a strict CSP meta tag in `app/layout.tsx`:

tsx

Copy code

`<head>
  <meta httpEquiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'; object-src 'none'; style-src 'self' 'unsafe-inline';"/>
</head>`

-   **HTTPS Upgrades:** For now, always attempt to load `https://` versions of URLs by checking if the user entered `http://` and rewriting to `https://` before calling `browser.load_url`. In future phases, implement a smarter approach or preload an `https-upgrade` list.

-   **Sandboxing CEF:** On Linux, launch CEF subprocesses with `--no-sandbox` disabled (i.e., use sandbox) and set appropriate flags. On Windows, run with AppContainer if supported by CEF. Detailed sandbox config depends on the platform and CEF version. For Phase 1, just ensure you are building in a mode that isolates renderer processes.

#### Step 9: Test on Multiple Platforms

-   Build and run on Linux:

    bash

    Copy code

    `cargo run`

    Confirm UI loads, navigation works.
-   Repeat on Windows (using MSVC toolchain) or macOS (Xcode tools installed).

If the UI appears, the navigation buttons trigger messages, and the browser engine loads the corresponding pages, Phase 1 goals are met.

#### Step 10: Documentation and Iteration

-   Write a `README.md` explaining how to build and run.
-   Add comments in code explaining the IPC mechanism and directory structure.
-   Create a `CONTRIBUTING.md` for future collaborators.
-   Set up GitHub Actions or another CI to run `cargo test` and `npm run build` for each commit.

* * * * *

### Results of Phase 1

By the end of these steps:

-   You have a Rust-based core application that starts CEF and displays a window.
-   You have a Next.js `app` directory UI compiled into static assets and served locally, displayed in the main browser window.
-   The UI can send basic commands (navigate, back, forward, refresh) to the Rust core, which then controls CEF.
-   A minimal CSP and attempted sandboxing provide a security baseline.
-   You have a reproducible build environment and a rough architectural blueprint for future phases (adding Tor, multiple engines, advanced privacy features).