use std::path::Path;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts{
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)] // 验证输入的文件是否存在，可以在这里加一个函数来验证文件是否存在
    pub input: String,
    #[arg(short, long, default_value = "ouotput.json")]
    pub output: Option<String>,
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short = 'd', long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok((filename).into())
    } else {
        Err("文件不存在")
    }
}