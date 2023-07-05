// 引入所需的库和模块
use nfd2::Response;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::fs;
use std::path::Path;
use pdf_extract;

/// # 开启文档
/// 这个函数用于打开用户选择的文件，并将文件内容复制到剪贴板中。
/// 文件可以是文本文件，也可以是PDF文件。
/// 对于文本文件，将直接读取文件的内容；
/// 对于PDF文件，将使用`pdf_extract`库来提取文件中的文本。
/// 如果用户选择了多个文件，或者取消了文件选择，将返回一个包含错误消息的Result。
/// 该函数是异步的，需要在异步环境中运行。
/// # 返回值
/// 如果文件打开成功，返回`Ok(1)`；
/// 如果发生错误，返回`Err(String)`，其中`String`是错误消息。
#[tauri::command]
pub async fn open_document() -> Result<u8, String> {
    let result = nfd2::open_file_dialog(Some("pdf,txt"), None).map_err(|e| e.to_string())?;
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e| e.to_string())?;

    match result {
        Response::Okay(file_path) => {
            let path = Path::new(&file_path);
            let mut content = String::new();
            if path.extension().and_then(|s| s.to_str()) == Some("pdf") {
                let bytes = fs::read(&file_path).map_err(|e| e.to_string())?;
                let content = pdf_extract::extract_text_from_mem(&bytes).map_err(|e| e.to_string())?;
                let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e| e.to_string())?;
            } else {
                // Read the file content
                content = tokio::fs::read_to_string(&file_path).await.map_err(|e| e.to_string())?;
            }

            // Copy the file content to the clipboard
            ctx.set_contents(content).map_err(|e| e.to_string())?;
            Ok(1)
        },

        Response::OkayMultiple(files) => {
          // 返回一个错误消息，说明你的函数不支持处理多个文件
          Err("好多文件啊,不知道要选哪个了呢".to_string())
        },

        Response::Cancel => {
            // 返回一个错误消息，说明用户已经取消了文件选择
          Err("文件打开已取消!".to_string())
        }
    }
}
