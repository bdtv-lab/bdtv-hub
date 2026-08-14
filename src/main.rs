mod action;

use anyhow::Result;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

use crate::action::{Action, handle_action};

const ADDR: &str = "0.0.0.0:7497";

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(ADDR).await?;
    println!("listening on ws://{ADDR}");

    while let Ok((stream, peer)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle(stream).await {
                eprintln!("{peer} error: {e}");
            }
            println!("{peer} disconnected");
        });
    }

    Ok(())
}

async fn handle(stream: TcpStream) -> Result<()> {
    let peer = stream.peer_addr()?;
    let mut ws = tokio_tungstenite::accept_async(stream).await?;
    println!("{peer} connected");

    while let Some(msg) = ws.next().await {
        match msg? {
            Message::Text(text) => {
                println!("{peer} -> {text}");

                let action: Action = serde_json::from_str(&text)?;

                handle_action(action)?;
            }
            Message::Binary(_) => {
                // println!("{peer} -> {} bytes", bin.len());
                // ws.send(Message::Binary(bin)).await?;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    Ok(())
}
