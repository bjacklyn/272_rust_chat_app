use wasm_bindgen::prelude::*;
use web_sys::{WebSocket, MessageEvent, HtmlElement, HtmlInputElement};
use serde::{Deserialize};
use serde_json::from_str;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Deserialize)]
struct ChatMessage {
    user_id: String,
    message: String,
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    // Connect to the WebSocket server
    let ws = WebSocket::new("ws://localhost:3000/ws")?;

    let user_id = Rc::new(RefCell::new("Unknown".to_string()));

    // Set up the message event listener
    let onmessage_callback = {
        let user_id = Rc::clone(&user_id);
        Closure::wrap(Box::new(move |event: MessageEvent| {
            let data = event.data();
            let data_string = data.as_string().unwrap_or_default();

            match from_str::<ChatMessage>(&data_string) {
                Ok(chat_message) => {
                    if chat_message.message == "This is your user_id" {
                        *user_id.borrow_mut() = chat_message.user_id;
                        return;
                    }

                    // Update the DOM with the new message
                    let messages = web_sys::window().unwrap().document().unwrap()
                        .get_element_by_id("messages").unwrap();
                    let message_element = messages.owner_document().unwrap()
                        .create_element("div").unwrap();
                    message_element.set_inner_html(&format!("{}: {}", chat_message.user_id, chat_message.message));

                    let message_element = message_element.dyn_into::<HtmlElement>().unwrap();

                    // Add message class self vs. other depending on who sent the message.
                    if chat_message.user_id == *user_id.borrow() {
                        message_element.class_list().add_1("self-message").unwrap();
                    } else {
                        message_element.class_list().add_1("other-message").unwrap();
                    }

                    messages.append_child(&message_element).unwrap();
                    messages.set_scroll_top(messages.scroll_height());
                },
                Err(e) => {
                    web_sys::console::log_1(&format!("Failed to parse message: {:?}, error: {:?}", data_string, e).into());
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>)
    };

    // Set the WebSocket message event listener
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
