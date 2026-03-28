use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write}; // 增加了 Write trait
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // 1. 打开文件
    let file = File::open(&args.path)
        .with_context(|| format!("could not open file `{}`", args.path.display()))?;
    
    let reader = BufReader::new(file);

    // --- 优化输出部分 ---
    // 获取标准输出的句柄并加上缓冲区
    // 这样可以减少系统调用的次数，在大批量匹配时性能提升显著
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock()); 

    // 2. 遍历每一行
    for line in reader.lines() {
        let line = line.with_context(|| "error reading line from file")?;
        
        if line.contains(&args.pattern) {
            // 使用 writeln! 替代 println!
            // 注意：如果写入失败，这里会返回错误
            writeln!(handle, "{}", line).with_context(|| "failed to write to stdout")?;
        }
    }

    // 确保所有缓冲区内容都已刷入终端
    handle.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli() {
        let args = Cli::parse_from(["grrs", "test_pattern", "test_file.txt"]);
        assert_eq!(args.pattern, "test_pattern");
        assert_eq!(args.path, std::path::PathBuf::from("test_file.txt"));
    }
}