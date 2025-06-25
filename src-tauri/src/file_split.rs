use crate::error::Error;
use crate::progress_payload::ProgressPayload;
use std::path::Path;
use tauri::{Emitter, Window};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

/// 获取文件行数
///
/// # Arguments
///
/// * `file_path` - 文件路径
///
/// # Returns
///
/// * `Result<usize, Error>` - 文件行数
#[tauri::command]
pub async fn get_line_count(file_path: &str) -> Result<usize, Error> {
    let file = File::open(file_path).await?;
    let reader = BufReader::new(file);
    let mut count = 0;

    let mut lines = reader.lines();
    while let Some(_) = lines.next_line().await? {
        count += 1;
    }
    Ok(count)
}

#[tauri::command]
pub async fn file_split(
    file_path: &str,
    output_dir: &str,
    chunk_size: i32,
    prefix: &str,
    window: Window,
) -> Result<(), Error> {
    let file = File::open(file_path).await?;

    // 处理输出路径 若未指定 则默认 file 同级目录
    let output_path = if output_dir.is_empty() {
        Path::new(file_path).parent().unwrap()
    } else {
        Path::new(output_dir)
    };

    // 已处理行数 用于拆分计算
    let mut current_line = 0;
    // 已处理行数 用于进度条控制
    let mut processed_line = 0;
    // 拆分序号
    let mut chunk_number = 1;
    // 创建首个拆分出的文件
    let mut writer = create_writer(prefix, chunk_number, output_path).await?;

    // 读取文件内容
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        writer.write_all(format!("{}\n", line).as_bytes()).await?;

        current_line += 1;
        processed_line += 1;

        // 更新进度条
        if current_line % 1000 == 0 {
            let _ = window.emit(
                "file_split_progress_update",
                ProgressPayload {
                    progress: processed_line,
                },
            );
            // tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }

        // 已完成当前分片拆分 新建文件
        if current_line >= chunk_size {
            writer.flush().await?;
            chunk_number += 1;
            writer = create_writer(&prefix, chunk_number, output_path).await?;
            current_line = 0;
        }
    }

    writer.flush().await?;
    Ok(())
}

/// 创建文件写入器
///
/// # Arguments
///
/// * `prefix` - 文件名前缀
/// * `chunk_number` - 拆分序号
/// * `output_path` - 输出路径
///
/// # Returns
///
/// * `Result<BufWriter<File>, Error>` - 写入器
async fn create_writer(
    prefix: &str,
    chunk_number: usize,
    output_path: &Path,
) -> Result<BufWriter<File>, Error> {
    let filename = if prefix.is_empty() {
        format!("{}_{:04}.txt", "chunk", chunk_number)
    } else {
        format!("{}_{:04}.txt", prefix, chunk_number)
    };
    let path = output_path.join(filename);
    let file = File::create(path).await?;
    return Ok(BufWriter::new(file));
}
