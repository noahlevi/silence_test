# Rust WebAssembly WebSocket Example

## Project Overview
This project demonstrates how to create a simple WebAssembly (Wasm) module in Rust that communicates with a WebSocket server. The WebAssembly module exports a single function, `wsPing`, which establishes a WebSocket connection, sends a message to the server, receives a response, and returns it. A basic HTML UI serves as the interface to call this function and display the results.

## Table of Contents

- [Project Overview](#project-overview)
- [Project Structure](#project-structure)
- [WebAssembly Module Logic](#webassembly-module-logic)
- [WebSocket Server Logic](#websocket-server-logic)
- [Setup](#setup)
- [Test](#test)

## Prerequisites

Before you begin, ensure you have the following installed on your machine:

- Rust (with `cargo` and `rustup`): [Install Rust](https://www.rust-lang.org/tools/install)
- Node.js (including npm): [Download Node.js](https://nodejs.org/)
- `wasm-pack`: Install via cargo
```bash
  cargo install wasm-pack
```

## WebAssembly Module Logic

1. **WebSocket Initialization**:
   - The function `wsPing(endpoint: String, message: String)` is the entry point that accepts two parameters: `endpoint`, which is the WebSocket server URL, and `message`, the message to send to the server.
   - A new `WebSocket` object is created with the specified `endpoint`.

2. **Handling Messages**:
   - A closure is defined to handle incoming messages from the WebSocket. This closure listens for messages and sends them to a message channel (an `mpsc` channel) for asynchronous processing.
   - The closure is wrapped with `Closure::wrap`, making it callable in the JavaScript environment.

3. **Sending the Message**:
   - The specified message is sent to the WebSocket server using the `send_with_str` method.

4. **Receiving the Response**:
   - The function waits for a response using an asynchronous future. It listens for the next message on the channel and retrieves it once the server responds.
   - The received message is then returned as a result to the caller.

5. **Error Handling**:
   - The function uses `Result<String, JsValue>` as the return type, allowing it to handle errors gracefully. If any step fails (e.g., connection errors or message sending issues), an appropriate error is returned.

## WebSocket Server Logic

1. **Setup**:
   - The WebSocket server is built using Node.js and the `ws` library. The server listens on a specified port (e.g., 8080).

2. **Connection Handling**:
   - When a client connects, the server logs the connection and prepares to listen for incoming messages.

3. **Message Processing**:
   - Each message received from the client is logged, and the server immediately responds with a corresponding "Echo" message, allowing the client to verify that the communication was successful.

4. **Connection Management**:
   - The server logs disconnections, helping to monitor client activity and maintain robust server operation

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


### 1)Clone the Repository:

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

## Test
From the root directory, initialize npm and install necessary packages:
```bash
cd task_2/client
npx tsc
node dist/test.js
```
