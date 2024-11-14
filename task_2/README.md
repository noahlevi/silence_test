# Rust WebAssembly WebSocket Example

This project demonstrates how to implement a WebSocket client using Rust compiled to WebAssembly (Wasm) and a WebSocket server using TypeScript. The client sends messages to the server and receives responses in real-time.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Project Structure](#project-structure)
- [Setup](#setup)
- [Running the WebSocket Server](#running-the-websocket-server)
- [Running the Wasm Client](#running-the-wasm-client)
- [Testing the WebSocket Connection](#testing-the-websocket-connection)
- [License](#license)
- [Contributing](#contributing)

## Prerequisites

Before you begin, ensure you have the following installed on your machine:

- Rust (with `cargo` and `rustup`): [Install Rust](https://www.rust-lang.org/tools/install)
- Node.js (including npm): [Download Node.js](https://nodejs.org/)
- `wasm-pack`: Install via cargo
```bash
  cargo install wasm-pack
```

## Project Structure

wasm_websocket/                                                                      <br>
│                                                                                    <br>
├── pkg/                          # Output directory from wasm-pack build            <br>
│   ├── task_2.wasm               # Compiled Wasm binary                             <br>
│   └── task_2.js                 # JavaScript bindings for the Wasm module          <br>
│                                                                                    <br>
├── src/                          # Source directory for Rust code                   <br>
│   └── lib.rs                    # Rust code for WebSocket functionality            <br>
│
├── client/
│   ├── server.ts                 # TypeScript WebSocket server implementation       <br>
│   ├── test.ts                   # TypeScript client to test WebSocket interaction  <br>
│   ├── Cargo.toml                # Rust package manifest                            <br>
│   ├── tsconfig.json             # TypeScript configuration file                    <br>
│   └── package.json              # Node.js package configuration                    <br>
└── README.md                     # Project documentation                            <br>

### Explanation of Project Structure

- **pkg/**: Contains the output from the `wasm-pack` build process, including the compiled WebAssembly binary and JavaScript glue code.
- **src/**: Contains the Rust source code, including the main library (`lib.rs`) that handles WebSocket communication.
- **server.ts**: A TypeScript file that implements a WebSocket server.
- **test.ts**: A TypeScript file that tests the WebSocket functionality by acting as a client.
- **Cargo.toml**: The manifest file for the Rust project, specifying dependencies and project metadata.
- **package.json**: The package configuration file for the Node.js project, listing dependencies and scripts.
- **tsconfig.json**: TypeScript configuration file that defines compiler options and file inclusions.
- **README.md**: The documentation for the project, outlining setup instructions and project details.

## Setup


### 1) Clone the Repository:

```bash
git clone <repository_url>
cd wasm_websocket
```

### 2) Build the Rust Project:
Make sure you are in the project root directory and run:
```bash
wasm-pack build --target nodejs
```

### 3) Install Node.js Dependencies:
From the root directory, initialize npm and install necessary packages:
```bash
npm init -y
npm install typescript @types/node ws
npm install --save-dev ts-node @types/ws
```

### 4) Test
From the root directory, initialize npm and install necessary packages:
```bash
cd task_2/client
npx tsc
node dist/test.js
```
