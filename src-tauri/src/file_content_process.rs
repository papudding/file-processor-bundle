use crate::error::Error;
use rayon::prelude::*;
use regex::Regex;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::Mutex;
use tauri::{Emitter, Window};
use crate::progress_payload::ProgressPayload;

#[tauri::command]
pub async fn file_search(
    file_paths: &str,
    output_dir: &str,
    search_string: &str,
    regex: bool,
    parallel: bool,
    window: Window,
) -> Result<(), Error> {
    // 获取输出目录路径
    let output_dir = Path::new(output_dir);

    // 输入文件集合
    let in_files: Vec<&str> = file_paths.split(',').collect();

    // 按文件并发操作
    let count = Mutex::new(0);
    in_files.par_iter().try_for_each(|&file_path| {
        let output_path = output_dir.join(format!("{}-search.txt", file_path));

        let output_file = File::create(output_path)?;
        let mut writer = BufWriter::new(output_file);
        writeln!(writer, "行号: 内容")?;

        if regex {
            // 正则模式
            let re = Regex::new(search_string).unwrap();
            inner_search(file_path, &mut writer, parallel, move |line| {
                re.is_match(line)
            })?;
        } else {
            // 普通模式
            let search_strings: Vec<String> = search_string
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            inner_search(file_path, &mut writer, parallel, move |line| {
                search_strings.iter().any(|pat| line.contains(pat))
            })?;
        }

        // 刷新缓冲区
        writer.flush()?;
        
        // 更新进度条
        let mut count = count.lock().unwrap();
        *count += 1;
        let _ = window.emit(
            "file_content_process_progress_update",
            ProgressPayload { progress: *count },
        );
        Ok(())
    })
}
fn inner_search<W, M>(
    file_path: &str,
    writer: &mut W,
    parallel: bool,
    matcher: M,
) -> Result<(), Error>
where
    W: Write + Send,                             // 写入器需线程安全
    M: Fn(&str) -> bool + Send + Sync + 'static, // 闭包需满足线程安全
{
    if parallel {
        // 并行模式
        let file_content = fs::read_to_string(file_path)?;
        let writer_mutex = Mutex::new(writer);
        let file_lines: Vec<_> = file_content
            .lines()
            .enumerate()
            .map(|(i, line)| (i + 1, line))
            .collect();

        file_lines.par_iter().for_each(|(rownum, line)| {
            if matcher(line) {
                let mut guard = writer_mutex.lock().unwrap();
                if let Err(e) = writeln!(guard, "{}: {}", rownum, line) {
                    eprintln!("写入失败: {}", e);
                }
            }
        });
    } else {
        // 串行模式
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for (rownum, line) in reader.lines().enumerate() {
            let line = line?;
            if matcher(&line) {
                writeln!(writer, "{}: {}", rownum + 1, line)?;
            }
        }
    }
    Ok(())
}

/// 替换文件内容
#[tauri::command]
pub async fn file_replace(
    file_paths: &str,
    output_dir: &str,
    regex: bool,
    search_string: &str,
    replace_string: &str,
    window: Window,
) -> Result<(), Error> {
    // 获取输出目录路径
    let output_dir = Path::new(output_dir);

    // 输入文件集合
    let in_files: Vec<&str> = file_paths.split(',').collect();

    // 按文件并发操作
    let count = Mutex::new(0);
    in_files.par_iter().try_for_each(|&file_path| {
        let output_path = output_dir.join(format!("{}-replace.txt", file_path));
        let output_file = File::create(output_path)?;
        let mut writer = BufWriter::new(output_file);
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if regex {
                // 正则模式
                let re = Regex::new(search_string).unwrap();
                let line = re.replace_all(&line, replace_string);
                writeln!(writer, "{}", line)?;
            } else {
                // 普通模式
                let line = line.replace(search_string, replace_string);
                writeln!(writer, "{}", line)?;
            }
        }

        // 更新进度条
        let mut count = count.lock().unwrap();
        *count += 1;
        let _ = window.emit(
            "file_content_process_progress_update",
            ProgressPayload { progress: *count },
        );
        Ok(())
    })
}
