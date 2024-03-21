use std::error::Error;
use tokio::net::TcpListener;
use tokio::stream::StreamExt;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

struct WebSocketServer {
    addr: String,
}

impl WebSocketServer {
    pub fn new(addr: String) -> WebSocketServer {
        WebSocketServer { addr }
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(&self.addr).await?;

        println!("Server started on {}", self.addr);

        loop {
            let (stream, _) = listener.accept().await?;

            tokio::spawn(async move {
                if let Err(e) = self.handle_connection(stream).await {
                    eprintln!("Error: {}", e);
                }
            });
        }
    }

    async fn handle_connection(&self, stream: tokio::net::TcpStream) -> Result<(), Box<dyn Error>> {
        let mut websocket = accept_async(stream).await?;
        println!("New WebSocket connection established");

        while let Some(Ok(msg)) = websocket.next().await {
            match msg {
                Message::Text(text) => {
                    println!("Received message: {}", text);
                    websocket.send(Message::Text(text)).await?;
                }
                Message::Close(_) => {
                    println!("WebSocket connection closed");
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
}