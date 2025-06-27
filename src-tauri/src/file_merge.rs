use crate::error::Error;
use crate::progress_payload::ProgressPayload;
use rayon::prelude::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Mutex;
use tauri::{Emitter, Window};

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks

#[tauri::command]
pub async fn file_merge(
    file_paths: &str,
    output_dir: &str,
    output_file_name: &str,
    parallel: bool,
    window: Window,
) -> Result<(), Error> {
    // 获取输出文件
    let output_dir = Path::new(output_dir);
    let output_path = if output_file_name.is_empty() {
        output_dir.join("bundle.txt")
    } else {
        output_dir.join(output_file_name)
    };
    // 输入文件集合
    let in_files: Vec<&str> = file_paths.split(',').collect();
    if !parallel {
        // 顺序合并
        merge_files_sequential(&in_files, &output_path, &window)?;
    } else {
        // 并行合并
        merge_files_parallel(&in_files, &output_path, &window)?;
    }
    Ok(())
}

fn merge_files_sequential(
    in_files: &Vec<&str>,
    output_path: &Path,
    window: &Window,
) -> Result<(), Error> {
    let mut count = 0;
    let mut output = File::create(output_path)?;
    in_files.iter().try_for_each(|&file_path| {
        let mut file = File::open(file_path)?;
        let mut buffer = vec![0; CHUNK_SIZE];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            let chunk = buffer[..bytes_read].to_vec();
            output.write_all(&chunk)?;
        }
        // 更新进度条
        count += 1;
        let _ = window.emit(
            "file_merge_progress_update",
            ProgressPayload { progress: count },
        );

        Ok(())
    })
}

fn merge_files_parallel(
    in_files: &Vec<&str>,
    output_path: &Path,
    window: &Window,
) -> Result<(), Error> {
    let output = Mutex::new(File::create(output_path)?);
    let count = Mutex::new(0);
    in_files.par_iter().try_for_each(|&file_path| {
        let mut file = File::open(file_path)?;
        let mut buffer = vec![0; CHUNK_SIZE];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            let chunk = buffer[..bytes_read].to_vec();
            output.lock().unwrap().write_all(&chunk)?;
        }
        // 更新进度条
        let mut count = count.lock().unwrap();
        *count += 1;
        let _ = window.emit(
            "file_merge_progress_update",
            ProgressPayload { progress: *count },
        );
        Ok(())
    })
}
