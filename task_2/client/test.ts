import { wsPing } from "/home/levi/Dev/silence_test/task_2/pkg/task_2.js"; // Adjust the path depending on the build output

async function testWSPing() {
    try {
        const SERVER = "ws://localhost:8081"; // WebSocket server address
        const message = "Hello, Server!"; // Message to send

        // Call the wsPing function and await its response
        const response = await wsPing(SERVER, message);

        // Log the received response in the console
        console.log("Received response:", response);
    } catch (error) {
        // Catch any errors during initialization or WebSocket communication
        console.error("Error initializing module or pinging WebSocket:", error);
    }
}

// Execute the test function
testWSPing();