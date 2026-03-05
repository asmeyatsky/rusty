use serde::Serialize;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

pub async fn read_line() -> Option<String> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();
    match reader.read_line(&mut line).await {
        Ok(0) => None,
        Ok(_) => Some(line.trim().to_string()),
        Err(_) => None,
    }
}

pub async fn write_response<T: Serialize>(response: &T) {
    let json = serde_json::to_string(response).unwrap();
    let mut stdout = io::stdout();
    let _ = stdout.write_all(json.as_bytes()).await;
    let _ = stdout.write_all(b"\n").await;
    let _ = stdout.flush().await;
}
