use axum::{
    extract::ws::{self, Message, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures_util::{stream::StreamExt, stream::SplitSink, SinkExt};
use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::{broadcast, Mutex as TokioMutex};
use tower_http::services::fs::ServeDir;
use uuid::Uuid;

//#[derive(Serialize)]
struct ChatMessage {
    user_id: String,
    message: String,
}

type Clients = Arc<TokioMutex<HashMap<String, Arc<TokioMutex<SplitSink<ws::WebSocket, ws::Message>>>>>>;

async fn chat_page() -> impl IntoResponse {
    Html(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Chat App</title>
    <style>
        #messages {
            border: 1px solid black;
            height: 300px;
            overflow-y: scroll;
        }
        #input {
            width: 100%;
        }
    </style>
</head>
<body>
    <div id="messages"></div>
    <input id="input" type="text" />
    <script type="module">
        import init from './wasm_frontend.js';

        async function run() {
            await init();
        }

        run();
    </script>
</body>
</html>
        "#,
    )
}

async fn handle_connection(
    socket: WebSocket,
    clients: Clients,
    tx: broadcast::Sender<String>,
) {
    let (sender, mut receiver) = socket.split();
    let user_id = Uuid::new_v4().to_string();

    {
        let mut clients = clients.lock().await;
        clients.insert(user_id.clone(), Arc::new(TokioMutex::new(sender)));
    }

    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(text) => {
                println!("Received message: {}", text);
                let _ = tx.send(text.clone());
            }
            Message::Close(_) => {
                break; // Exit the loop on close message
            }
            _ => {}
        }
    }

    // Remove the client when they disconnect
    {
        let mut clients = clients.lock().await;
        clients.remove(&user_id.to_string());
    }
}

async fn handle_websocket(
    ws: WebSocketUpgrade,
    clients: Clients,
    tx: broadcast::Sender<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_connection(socket, clients, tx))
}


async fn broadcast_messages(
    tx: broadcast::Sender<String>,
    clients: Clients,
) {
    let mut rx = tx.subscribe();

    loop {
        let message = match rx.recv().await {
            Ok(msg) => msg,
            Err(_) => break, // Handle errors (e.g., if the sender is dropped)
        };

        println!("Broadcasting message: {}", message);

        let clients = clients.lock().await;
        for (_id, client) in clients.iter() {
            let mut sender = client.lock().await;
            let _ = sender.send(Message::Text(message.clone())).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let clients = Arc::new(TokioMutex::new(HashMap::new()));
    let (tx, _rx) = broadcast::channel::<String>(100);

    let app = Router::new()
        .route("/", get(chat_page))
        .route("/ws", get({
            let clients = Arc::clone(&clients);
            let tx = tx.clone();
            move |ws: WebSocketUpgrade| handle_websocket(ws, clients.clone(), tx.clone())
        }))
        .fallback_service(ServeDir::new("/home/bjacklyn/sjsu_272/272_4/wasm_frontend/pkg"));

    tokio::spawn(broadcast_messages(tx.clone(), clients));

    // Run the Axum server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
