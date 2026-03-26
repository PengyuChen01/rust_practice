use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::{Context, Result};
// search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // 1. 打开文件：如果失败，会提示具体是哪个路径打不开
    let file = File::open(&args.path)
        .with_context(|| format!("could not open file `{}`", args.path.display()))?;
    
    let reader = BufReader::new(file);

    // 2. 遍历每一行
    for line in reader.lines() {
        // 使用 ? 快速处理读取行时的错误
        let line = line.with_context(|| "error reading line from file")?;
        
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(()) 
}
