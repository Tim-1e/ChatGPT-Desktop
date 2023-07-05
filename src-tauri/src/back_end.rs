use std::process::{Command, Child};
use tokio::task;
use std::env;

/// # 启动后端服务
/// 通过Python执行指定的脚本来启动后端服务。
/// 这个函数会改变当前的工作目录，因此在启动服务之后，需要恢复原来的工作目录。
/// 注意：此函数为实验函数,函数中的路径为硬编码，可能需要根据实际情况进行调整。
/// # 返回
/// 返回一个`Child`类型的值，表示后端服务的进程。
pub async fn start_backend()->Child{
    let (tx, rx) = tokio::sync::oneshot::channel();
    
    task::spawn_blocking(move || {
        //python位置与变量设置
        let python_executable = "E:\\DiffSinger\\sovits4.0\\Sovits\\env\\python.exe";
        let script_path = "E:\\DiffSinger\\sovits4.0\\Sovits\\meowChat.py";

        let current_dir = env::current_dir().unwrap();
        let sovits_home = current_dir.join("E:\\DiffSinger\\sovits4.0\\Sovits");
        // let sovits_util = current_dir.join("E:\\DiffSinger\\sovits4.0\\Sovits\\meowChat_util");

        env::set_current_dir(sovits_home).unwrap();
        //子线程启动python后端
        let mut child = Command::new(python_executable)
            .arg(script_path)
            .spawn()
            .expect("failed to start backend");

        child.wait().expect("backend process encountered an error");
        env::set_current_dir(current_dir).unwrap();
    });

    rx.await.expect("failed to start backend")
}
