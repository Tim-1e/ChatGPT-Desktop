use std::fs::File;
use std::io::{BufWriter, Write};
use std::{sync::Arc, time::Duration};
use std::time::SystemTime;
use tokio::{sync::Mutex, net::TcpStream};
use tauri::Manager;
use base64::{encode, decode};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::io::prelude::*;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use futures::{channel::mpsc, SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream, MaybeTlsStream};
use url::Url;
use hound;
use samplerate::{convert, ConverterType};

/// # IatRecorder
/// 这个结构体代表一个与科大讯飞在线语音转文字服务进行交互的录音器。它包含一些如应用ID、API密钥、API秘钥、语言和口音等信息，
/// 以及一些内部状态、音频数据和转化结果等数据。所有这些数据都被包装在`Mutex`中，以便于在异步上下文中进行共享和修改。
pub struct IatRecorder {
    // 科大讯飞的应用ID、API密钥和API秘钥
    app_id: String,
    api_key: String,
    api_secret: String,
    // 语言和口音
    language: String,
    accent: String,
    // 录音器的内部状态
    status: Mutex<String>,
    // 音频流的引用
    stream_ref: Mutex<Vec<u8>>,
    // 实际的音频数据
    audio_data: Mutex<Vec<u8>>,
    // 转化结果（完整的和临时的）
    result_text: Mutex<String>,
    result_text_temp: Mutex<String>,
    // 与科大讯飞服务的WebSocket连接
    websocket: Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>
}

type HmacSha256 = Hmac<Sha256>;

impl IatRecorder {
    /// # 新建一个 IatRecorder
    /// 这个异步方法新建并初始化一个`IatRecorder`。参数中的语言和口音是可选的，如果没有指定，则会使用默认的中文和普通话。
    pub async fn new(app_id: String, api_key: String, api_secret: String, language: Option<String>, accent: Option<String>) -> Self {
        let recorder = Self {
            app_id: app_id,
            api_key: api_key,
            api_secret: api_secret,
            language: language.unwrap_or("zh_cn".to_string()),
            accent: accent.unwrap_or("mandarin".to_string()),
            status: Mutex::new("null".to_string()),
            stream_ref: Mutex::new(Vec::new()),
            audio_data: Mutex::new(Vec::new()),
            result_text: Mutex::new("".to_string()),
            result_text_temp: Mutex::new("".to_string()),
            websocket: Mutex::new(None),
        };
        recorder.init().await;
        recorder
    }

    /// # 获取WebSocket连接的URL
    /// 这个异步方法用于获取与科大讯飞服务建立WebSocket连接的URL。
    async fn get_websocket_url(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = "wss://iat-api.xfyun.cn/v2/iat";
        let host = "iat-api.xfyun.cn";
        let date = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let algorithm = "hmac-sha256";
        let headers = "host date request-line";
        let signature_origin = format!("host: {}\ndate: {}\nGET /v2/iat HTTP/1.1", host, date);
        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(signature_origin.as_bytes());
        let signature = mac.finalize().into_bytes();
        let authorization_origin = format!(
            r#"api_key="{}", algorithm="{}", headers="{}", signature="{}""#,
            self.api_key,
            algorithm,
            headers,
            base64::encode(&signature)
        );
        let authorization = base64::encode(&authorization_origin);
        Ok(format!(
            "{}?authorization={}&date={}&host={}",
            url, authorization, date, host
        ))
    }

    /// # 初始化
    /// 这个异步方法用于初始化`IatRecorder`。它会检查应用ID、API密钥和API秘钥等信息是否已经被正确地设置。
    async fn init(&self) {
        // Check configuration information
        if self.app_id.is_empty() || self.api_key.is_empty() || self.api_secret.is_empty() {
            panic!("请正确配置【迅飞语音听写（流式版）WebAPI】服务接口认证信息！");
        }
    }
    
    /// # 设置状态
    /// 这个异步方法用于设置`IatRecorder`的内部状态。
    async fn set_status(&self, status: &str) {
        println!("set to {}",status);
        let mut status_guard: tokio::sync::MutexGuard<String> = self.status.lock().await;
        *status_guard = status.to_string();
    }

    /// # 设置转化结果
    /// 这个异步方法用于设置转化的结果，包括完整的和临时的。
    async fn set_result_text(&self, result_text: Option<String>, result_text_temp: Option<String>) {
        if let Some(rt) = result_text {
            let mut result_text_guard = self.result_text.lock().await;
            *result_text_guard = rt;
        }
        if let Some(rt_temp) = result_text_temp {
            let mut result_text_temp_guard = self.result_text_temp.lock().await;
            *result_text_temp_guard = rt_temp;
        }
    }

    /// # 设置参数
    /// 这个方法用于设置语言和口音的参数。参数是可选的，如果没有指定，那么对应的值将不会改变。
    fn set_params(&mut self, language: Option<String>, accent: Option<String>) {
        if let Some(lang) = language {
            self.language = lang;
        }
        if let Some(acc) = accent {
            self.accent = acc;
        }
    }

    /// # 将数据转换为Base64格式
    /// 这个方法用于将给定的字节序列转换为Base64格式的字符串。
    fn to_base64(buffer: &[u8]) -> String {
        base64::encode(buffer)
    }

    /// 建立 WebSocket 连接
    /// 该异步函数创建了一个WebSocket连接，将状态设置为'init'，并开始了录音，WebSocket循环和接收任务。当所有任务完成后，函数返回。
    async fn connect_websocket(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("begin connect");
        let url = self.get_websocket_url().await?;
        println!("our url is{}",&url);
        let (socket, _) = connect_async(Url::parse(&url)?).await?;
        {
            let mut websocket_guard = self.websocket.lock().await;
            *websocket_guard = Some(socket);
        }

        println!("success");
        self.set_status("init").await;
    
        let (recording_task, send_task, receive_task) = tokio::join!(
            self.start_recording(),
            self.web_socket_loop(),
            self.web_socket_res_loop(),
        );
    
        // 等待三个任务完成
        recording_task?;
        send_task?;
        receive_task?;

        Ok(())
    }
    
    /// WebSocket 循环
    /// 该异步函数首先等待500毫秒，然后开始 WebSocket 循环，发送音频数据。
    async fn web_socket_loop(&self)-> Result<(), Box<dyn std::error::Error>> {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        print!("begin websocket loop");
        self.web_socket_send().await?;
        Ok(())
    }

    /// WebSocket 响应循环
    /// 该异步函数开启了一个定时器，每40毫秒进行一次循环，获取 WebSocket 的消息，处理消息，直到状态不再为 'ing' 时停止循环。
    async fn web_socket_res_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = tokio::time::interval(Duration::from_millis(40));
        loop {
            interval.tick().await;
    
            let status = self.status.lock().await.clone();
            if status != "ing" {
                break;
            }
    
            let message = match self.websocket.lock().await.as_mut() {
                Some(socket) => socket.next().await,
                None => break,
            };
    
            match message {
                Some(Ok(message)) => {
                    println!("get message");
                    let message_data = message.to_text().unwrap();
                    self.web_socket_res(message_data.to_owned()).await;
                },
                Some(Err(e)) => {
                    println!("WebSocket error: {:?}", e);
                    break;
                },
                None => break,
            }
        }
    
        self.stop().await;
        Ok(())
    }

    /// 开始录音
    /// 该异步函数首先获取默认的音频输入设备，然后开始播放录音流。当状态变为 'end' 时，停止录音。
    async fn start_recording(&self) -> Result<(), Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let input_device = host.default_input_device().ok_or("Failed to get default input device")?;
        
        let (mut sender, mut receiver) = mpsc::channel(1);
        let config: cpal::StreamConfig =cpal::StreamConfig{
            channels: 2,
            sample_rate: cpal::SampleRate(48000),
            buffer_size: cpal::BufferSize::Default,
        };
        
        let input_data_callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let _ = sender.try_send(data.to_vec());
        };
        let timeout = None;
        let stream = input_device.build_input_stream(&config, input_data_callback, handle_stream_error,timeout)?;
        
        stream.play()?;
        self.set_status(&"ing".to_string()).await;
        println!("come to start recording");
        
        
        while let Some(audio_chunk) = receiver.next().await {
            if self.status.lock().await.as_str() == "end" {
                break;
            }
    
            let resample_chunk: Vec<f32>=convert(48000,16000,2,ConverterType::SincBestQuality, &audio_chunk).unwrap();

            let mono_audio_data: Vec<f32> = stereo_to_mono_f32(resample_chunk);

            let audio_data_u16: Vec<u16> = mono_audio_data
            .into_iter()
            .map(|sample| (sample * (i16::MAX as f32)).round() as u16)
            .collect();

            let audio_data_u8: Vec<u8> = audio_data_u16
            .into_iter()
            .flat_map(|sample| sample.to_ne_bytes().to_vec())
            .collect();
            let mut audio_data_guard = self.audio_data.lock().await;
            audio_data_guard.extend_from_slice(&audio_data_u8);
        }

        stream.pause()?;
        Ok(())
    }

    /// WebSocket 发送
    /// 该异步函数锁定 WebSocket，并发送音频数据，每40毫秒发送一次，直到状态不再为 'ing'。
    async fn web_socket_send(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut websocket_guard = self.websocket.lock().await;
        let websocket = websocket_guard.as_mut().ok_or("WebSocket is not connected")?;
        
        let status = self.status.lock().await.clone();
        if status != "ing" {
            return Ok(());
        }
    
        let mut audio_data_guard = self.audio_data.lock().await;
        let audio_data = audio_data_guard.drain(0..1280).collect::<Vec<u8>>();
    
        let params = json!({
            "common": {
                "app_id": self.app_id,
            },
            "business": {
                "language": self.language,
                "domain": "iat",
                "accent": self.accent,
                "vad_eos": 5000,
                "dwa": "wpgs"
            },
            "data": {
                "status": 0,
                "format": "audio/L16;rate=16000",
                "encoding": "raw",
                "audio": IatRecorder::to_base64(&audio_data)
            }
        });
    
        websocket.send(Message::Text(params.to_string())).await?;
    
        let mut interval = tokio::time::interval(Duration::from_millis(40));
    
        loop {
            interval.tick().await;
    
            let status = self.status.lock().await.clone();
            if status != "ing" {
                break;
            }
    
            let audio_data = audio_data_guard.drain(0..1280).collect::<Vec<u8>>();
    
            let params = json!({
                "data": {
                    "status": 1,
                    "format": "audio/L16;rate=16000",
                    "encoding": "raw",
                    "audio": IatRecorder::to_base64(&audio_data)
                }
            });
    
            websocket.send(Message::Text(params.to_string())).await?;
        }
    
        if status == "end" {
            let params = json!({
                "data": {
                    "status": 2,
                    "format": "audio/L16;rate=16000",
                    "encoding": "raw",
                    "audio": ""
                }
            });
    
            websocket.send(Message::Text(params.to_string())).await?;
        }
    
        Ok(())
    }
    
    /// WebSocket 响应
    /// 该异步函数处理从WebSocket收到的消息。如果数据中有语音转文本的结果，它会被保存下来。如果收到的消息指示语音转文本的过程已经结束，那么WebSocket会被关闭。
    async fn web_socket_res(&self, result_data: String) {
        let json_data: serde_json::Value = serde_json::from_str(&result_data).unwrap();
        if let Some(result) = json_data["data"]["result"].as_object() {
            let ws = result["ws"].as_array().unwrap();
            let mut str = String::new();
            for w in ws {
                str.push_str(&w["cw"][0]["w"].as_str().unwrap());
            }
            if let Some(pgs) = result["pgs"].as_str() {
                if pgs == "apd" {
                    self.set_result_text(Some(self.result_text_temp.lock().await.clone()), None).await;
                }
                self.set_result_text(None, Some(self.result_text.lock().await.clone() + &str)).await;
            } else {
                self.set_result_text(Some(self.result_text.lock().await.clone() + &str), None).await;
            }
        }
        if json_data["code"] == 0 && json_data["data"]["status"] == 2 {
            let mut websocket_guard = self.websocket.lock().await;
            if let Some(ws_stream) = websocket_guard.as_mut() {
                ws_stream.close(None).await;
            }
        }
        if json_data["code"] != 0 {
            let mut websocket_guard = self.websocket.lock().await;
            if let Some(ws_stream) = websocket_guard.as_mut() {
                ws_stream.close(None).await;
            }
        }
    }

    /// 启动录音器
    /// 该异步函数开始录音，并处理可能出现的错误。
    async fn recorder_start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.start_recording().await?;
        Ok(())
    }

    /// 停止录音器
    /// 该异步函数将状态设置为 'end'，以停止录音。
    async fn recorder_stop(&self) {
        self.set_status(&"end".to_string()).await;
    }

    /// 开始
    /// 这个异步函数尝试建立WebSocket连接并开始录音。如果出现任何错误，它会停止录音并打印错误消息。
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.connect_websocket().await.map_err(|err|async move {
            self.recorder_stop().await;
            print!("Error: {}", err);
        });
        Ok(())
    }

    /// 停止
    /// 这个异步函数让程序等待60秒，然后停止录音。(正常情况下我们会调用recorder_stop进入结束流程)
    pub async fn stop(&self) {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        self.recorder_stop().await;
    }

    /// 获取结果文本
    /// 这个异步函数返回语音转文本的结果。
    pub async fn get_result_text(&self) -> String {
        let result_text_guard = self.result_text.lock().await;
        result_text_guard.clone()
    }

    /// 保存音频
    /// 这个异步函数将音频数据写入到指定的文件中。
    pub async fn save_audio(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let audio_data_guard = self.audio_data.lock().await;
        let mut file = File::create(file_name)?;
        file.write_all(&audio_data_guard)?;
        Ok(())
    }
}

/// 处理音频流错误
/// 这个函数处理音频流中发生的错误，将错误信息打印到标准错误输出。
fn handle_stream_error(err: cpal::StreamError) {
    eprintln!("An error occurred on the input audio stream: {}", err);
}

/// 将立体声转为单声道
/// 这个函数接收一个立体声音频数据的向量，并返回一个单声道音频数据的向量。立体声数据的左右通道被混合以产生单声道数据。
/// 用来满足科大讯飞对于接收数据的pcm16规格要求
fn stereo_to_mono_f32(stereo_data: Vec<f32>) -> Vec<f32> {
    let mut mono_data: Vec<f32> = Vec::with_capacity(stereo_data.len() / 2);

    for samples in stereo_data.chunks_exact(2) {
        let left = samples[0];
        let right = samples[1];
        let mixed_sample = (left + right) / 2.0;
        mono_data.push(mixed_sample);
    }

    mono_data
}
