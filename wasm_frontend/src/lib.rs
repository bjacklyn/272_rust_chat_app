use wasm_bindgen::prelude::*;
use web_sys::{WebSocket, MessageEvent, HtmlInputElement};

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    // Connect to the WebSocket server
    let ws = WebSocket::new("ws://localhost:3000/ws")?;

    // Set up the message event listener
    let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        let message = event.data().as_string().unwrap();
        // Update the DOM with the new message
        let messages = web_sys::window().unwrap().document().unwrap().get_element_by_id("messages").unwrap();
        let message_element = web_sys::Document::create_element(&messages.owner_document().unwrap(), "div").unwrap();
        message_element.set_inner_html(&message);
        messages.append_child(&message_element).unwrap();
        messages.set_scroll_top(messages.scroll_height());
    }) as Box<dyn FnMut(MessageEvent)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let document = web_sys::window().unwrap().document().unwrap();
    let input_element = document.get_element_by_id("input").unwrap();
    let input: HtmlInputElement = input_element.dyn_into::<HtmlInputElement>()?;

    let input_clone = input.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        if event.key() == "Enter" {
            let value = input_clone.value();
            ws.send_with_str(&value).unwrap();
            input_clone.set_value("");
        }
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    input.add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}
