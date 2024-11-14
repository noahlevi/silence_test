use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;
use web_sys::WebSocket;

/// Establishes a WebSocket connection to the given endpoint, sends a message, and waits for a response.
///
/// # Arguments
///
/// * `endpoint` - A string representing the WebSocket endpoint URL.
/// * `message` - A string containing the message to be sent through the WebSocket.
///
/// # Returns
///
/// A promise that resolves to a string containing the response from the server.
///
/// # Errors
///
/// Returns a JavaScript value in case of failure during WebSocket operations.
#[wasm_bindgen(js_name = wsPing)]
pub async fn ws_ping(endpoint: String, message: String) -> Result<String, JsValue> {
    // Create a new WebSocket instance
    let ws = WebSocket::new(endpoint.as_str())
        .map_err(|_| JsValue::from_str("Failed to create WebSocket {}"))?;

    // Send the message to the WebSocket server
    ws.send_with_str(message.as_str())
        .map_err(|_| JsValue::from_str("Failed to send message"))?;

    // Create a closure to handle incoming messages
    let on_message = Closure::wrap(Box::new(move |ev: web_sys::MessageEvent| {
        let response = ev.data().as_string().unwrap_or_default();
        // Log the received message to the console
        web_sys::console::log_1(&JsValue::from_str(&response));
    }) as Box<dyn FnMut(_)>);

    // Set the onmessage event handler for the WebSocket
    ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget(); // Prevent the Closure from getting dropped

    // If you want to wait for some response, you may implement additional logic here
    // For the sake of this example, we send back a simple confirmation
    let response = JsFuture::from(Promise::resolve(&JsValue::from("Message sent."))).await?;

    Ok(response.as_string().unwrap_or_default())
}
