import WebSocket, { Server } from 'ws';

// Create a WebSocket server
const wss: Server = new WebSocket.Server({ port: 8081 });

wss.on('connection', (ws: WebSocket) => {
    console.log('Client connected');  // Log when a client connects

    ws.on('message', (message: string) => {
        console.log(`Received message: ${message}`);
        // Echo the message back to the client
        ws.send(`Server response: ${message}`);
    });

    ws.on('close', () => {
        console.log('Client disconnected');  // Log when a client disconnects
    });
});

console.log('WebSocket server is listening on ws://localhost:8081');