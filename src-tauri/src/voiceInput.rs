use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex;

/// # 语音输入
/// `VoiceInput` 结构体代表一个与语音服务连接的客户端，负责处理音频流的输入。
/// 它包含一个 `TcpStream`，用于与服务器进行通信。
pub struct VoiceInput {
    stream: TcpStream,
}

impl VoiceInput {
    /// # 新建一个 VoiceInput
    /// 这个异步方法尝试与指定的服务器地址建立连接，如果成功则返回一个新的 `VoiceInput`。
    /// # 参数
    /// * `server_address`: 要连接的服务器地址。
    /// # 返回
    /// 返回一个 `Result`，成功时包含新的 `VoiceInput`，失败时包含 `std::io::Error`。
    pub async fn new(server_address: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(server_address).await?;
        Ok(VoiceInput { stream })
    }

    /// # 发送信息
    /// 这个异步方法尝试将字符串消息发送到连接的服务器。
    /// # 参数
    /// * `msg`: 要发送的消息。
    /// # 返回
    /// 返回一个 `Result`，成功时包含 `()`，失败时包含 `std::io::Error`。
    pub async fn send(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.stream.write_all(msg.as_bytes()).await
    }

    /// # 接收信息
    /// 这个异步方法尝试从连接的服务器接收消息。
    /// # 返回
    /// 返回一个 `Result`，成功时包含收到的字符串，失败时包含 `std::io::Error`。
    pub async fn recv(&mut self) -> Result<String, std::io::Error> {
        let mut buffer = [0; 8192];
        let n = self.stream.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        Ok(response)
    }
}

// 接下来是一些 Tauri 命令，用于在 Tauri 应用中处理语音输入。
// 它们都被标记为 `tauri::command`，因此可以从 JavaScript 中调用。
// 这些命令用于初始化 `VoiceInput`（并将其存储在 Tauri state 中）、发送和接收语音输入。
#[tauri::command]
pub async fn init_voice_input(
    state: tauri::State<'_, Arc<Mutex<Option<VoiceInput>>>>,
    server_address: String,
) -> Result<(), String> {
    match VoiceInput::new(&server_address).await {
        Ok(voice_input) => {
            let mut state = state.lock().await;
            *state = Some(voice_input);
            println!("VoiceInput initialized");
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to connect to server: {}", e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}


#[tauri::command]
pub async fn send_voice_input(
    state: tauri::State<'_, Arc<Mutex<Option<VoiceInput>>>>,
    msg: String,
) -> Result<(), String> {
    let mut state = state.lock().await;
    if let Some(voice_input) = state.as_mut() {
        voice_input.send(&msg).await.map_err(|e| format!("Failed to send message: {}", e))
    } else {
        Err("VoiceInput not initialized.".to_string())
    }
}

#[tauri::command]
pub async fn recv_voice_input(
    state: tauri::State<'_, Arc<Mutex<Option<VoiceInput>>>>,
) -> Result<String, String> {
    let mut state = state.lock().await;
    if let Some(voice_input) = state.as_mut() {
        voice_input.recv().await.map_err(|e| format!("Failed to receive response: {}", e))
    } else {
        Err("VoiceInput not initialized.".to_string())
    }
}
