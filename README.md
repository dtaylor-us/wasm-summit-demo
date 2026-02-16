Here’s a **clean, summit-ready README** you can drop straight into the repo.
It’s written so **another architect could clone it and understand the point in 2 minutes**, even if they’ve never touched WASM or Spin.

You can copy-paste this as `README.md`.

---

# WebAssembly on the Server – Spin Demo

This repository demonstrates **WebAssembly (WASM) running on the server** using **Spin** as the runtime.

The goal of this demo is **not** to replace traditional backend frameworks, but to show how **small, fast, sandboxed services** can be built and run **without containers or an operating system**.

---

## What This Demo Shows

In ~10 minutes, this app demonstrates:

* A server-side WebAssembly application running as an HTTP service
* Ultra-small deployment artifacts (`.wasm` instead of container images)
* **Capability-based security**:

  * Outbound HTTP is **blocked by default**
  * Explicit allowlisting enables network access
* How WASM differs architecturally from containers

---

## Architecture Overview

```
Client (curl / browser)
        |
        v
     Spin Runtime
        |
        v
  WebAssembly Module (.wasm)
        |
        +--> (optional) Outbound HTTP
              (only if explicitly allowed)
```

Key idea:

> **The runtime controls what the WebAssembly module is allowed to do.**

---

## Endpoints

| Endpoint | Description                                            |
| -------- | ------------------------------------------------------ |
| `/hello` | Simple response proving WASM can act as a web service  |
| `/quote` | Makes an outbound HTTP call (initially denied by Spin) |

---

## Prerequisites

You’ll need the following installed:

* **Spin CLI**
* **Rust + Cargo**
* Rust target: `wasm32-wasip1`
* Python (for the simple quote server)

### Install Spin

```bash
curl -fsSL https://spinframework.dev/downloads/install.sh | bash
```

Ensure Spin is on your PATH:

```bash
spin --version
```

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Add the WASM target:

```bash
rustup target add wasm32-wasip1
```

---

## Build the WebAssembly Module

```bash
spin build
```

This compiles the Rust code into a **WebAssembly module**:

```bash
ls -lh target/wasm32-wasip1/release/*.wasm
```

This `.wasm` file is the **deployable artifact**.

---

## Run the Quote Server (Outbound Dependency)

In a separate terminal:

```bash
mkdir quote-server && cd quote-server
echo '{"quote":"WASM: small, fast, sandboxed.","author":"Architecture Summit Demo"}' > quote.json
python3 -m http.server 3001
```

---

## Run the Spin App

```bash
spin up
```

Spin will start an HTTP server on port `3000`.

---

## Test the Endpoints

### `/hello` (works immediately)

```bash
curl http://127.0.0.1:3000/hello
```

Expected response:

```
Hello from WebAssembly 👋
```

---

### `/quote` (fails by design)

```bash
curl http://127.0.0.1:3000/quote
```

Expected error:

```
ErrorCode::HttpRequestDenied
```

This is intentional.

> **Outbound HTTP is denied by default.**

---

## Enable Outbound HTTP (The “Wow” Moment)

Edit `spin.toml` and add the allowlist under the component definition:

```toml
allowed_outbound_hosts = ["http://127.0.0.1:3001"]
```

Restart Spin:

```bash
spin up
```

Retry:

```bash
curl http://127.0.0.1:3000/quote
```

Now it succeeds and returns the JSON from the quote server.

---

## Why This Matters Architecturally

This demo highlights several important ideas:

* **Capability-based security** instead of broad OS permissions
* Smaller attack surface than containers
* Extremely fast startup times
* Language-agnostic runtime (Spin doesn’t care how the `.wasm` was built)

This pattern is especially useful for:

* Event-driven workloads
* Plugin or extension systems
* Edge compute
* Running untrusted or semi-trusted logic safely

---

## What This Is NOT

This demo is **not** suggesting that WASM replaces:

* Spring Boot
* Containers
* Long-running stateful services

Instead, it introduces **another architectural building block**.

---

## One-Sentence Summary

> **WebAssembly on the server lets you run tiny, fast, sandboxed services with explicit runtime permissions — without containers or an operating system.**

