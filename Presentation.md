# Slide 1 — Title

## WebAssembly on the Server

### A New Runtime Building Block

**Speaker notes:**

> Today I’m not pitching a framework or a platform replacement.
> I’m introducing a *new runtime primitive* that sits somewhere between containers and native code.
> The demo afterward will be intentionally small — because this technology shines when the scope is small.

---

# Slide 2 — The Problem Space

## Why Look Beyond Containers?

* Containers are powerful — but heavy
* JVM / CLR cold starts hurt event-driven workloads
* Security boundaries are OS-based, not capability-based
* Every service gets *far more permissions than it needs*

**Speaker notes:**

> Containers solved deployment and portability — but they also normalized “ship a mini operating system per service.”
> For some workloads, that’s fine.
> For others — event handlers, glue code, plugins — it’s overkill.
> WebAssembly is interesting because it attacks *that exact gap*.

---

# Slide 3 — What Is WebAssembly?

## WebAssembly (WASM)

* A **compiled binary format**
* Runs inside a **sandbox**
* Same artifact everywhere
* No direct access to OS, network, or filesystem

**Speaker notes:**

> WebAssembly started in browsers to make JavaScript-heavy apps faster.
> The key design decision was safety: WASM can’t touch the system unless explicitly allowed.
> That decision turns out to be extremely valuable on the server.

---

# Slide 4 — WASM on the Server

## Server-Side WebAssembly

* WASM runs inside a **runtime**
* The runtime decides:

  * network access
  * file access
  * environment access
* The deployable unit is a **.wasm file**, not a container image

**Speaker notes:**

> Think of WASM as the code, and the runtime as the guardrails.
> Unlike containers, where you block things after the fact, WASM starts with *nothing* and adds capabilities intentionally.

---

# Slide 5 — Enter Spin

## What Is Spin?

* A **WebAssembly runtime**
* Focused on **HTTP and event-driven workloads**
* Starts WASM modules as web services
* Enforces security via configuration, not code

**Speaker notes:**

> Spin is not Kubernetes.
> It’s not Docker.
> It’s simply a way to say: “Run this WebAssembly module and expose it as an HTTP service — safely.”

---

# Slide 6 — Mental Model Comparison

## How This Differs from Containers

| Containers         | WebAssembly + Spin    |
| ------------------ | --------------------- |
| OS-level isolation | Runtime-level sandbox |
| Large images       | Tiny binaries         |
| Broad permissions  | Explicit capabilities |
| Seconds to start   | Milliseconds to start |

**Speaker notes:**

> This isn’t about replacing containers.
> It’s about adding a **lighter-weight option** when containers are too much.
> Especially when startup time, density, or security boundaries matter.

---

# Slide 7 — Capability-Based Security

## The Key Architectural Idea

* No outbound network by default
* No filesystem by default
* No environment access by default
* Everything must be **explicitly granted**

**Speaker notes:**

> This is the most important slide.
> With WASM, *policy is enforced by the runtime*, not just conventions or documentation.
> You’ll see this live in the demo when a network call fails — on purpose.

---

# Slide 8 — Where This Fits (and Doesn’t)

## Good Fit

* Event handlers
* API glue / adapters
* Plugins & extensions
* Edge workloads
* Running untrusted logic safely

## Poor Fit

* Large monoliths
* Heavy frameworks
* Long-running stateful services

**Speaker notes:**

> If you’re thinking “Could I run Spring Boot in this?” — that’s a red flag.
> This is a scalpel, not a hammer.

---

# Slide 9 — What You’re About to See

## Demo Walkthrough

1. A WASM service responding to HTTP
2. An outbound HTTP call that **fails**
3. A one-line config change
4. The same call **succeeds**

**Speaker notes:**

> The demo is intentionally boring in functionality and interesting in behavior.
> Watch what fails, *why* it fails, and how little it takes to change the outcome.

---

# Slide 10 — Takeaway

## One Architectural Takeaway

> **WebAssembly on the server introduces a new unit of deployment:
> small, fast, sandboxed code with explicit runtime permissions.**

**Speaker notes:**

> You don’t need to adopt this everywhere.
> But once you see it, you’ll start noticing places where containers are doing more work than necessary.

---

## Optional Closing Line (if you want a strong handoff)

> “Now let’s break it — and then fix it — live.”
