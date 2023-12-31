/*use std::error::Error;
use std::io::read_to_string;

#[test]
fn main() -> Result<(), Box<dyn Error>>{
    //同一个地方使用不同报错
    //错误返回
    let html = render()?;
    println!("{}", html);
    Ok(())
}
fn render() -> Result<String, Box<dyn Error>> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}*/

use std::fs::read_to_string;
#[test]
fn main() -> Result<(), MyError> {
    let html = render()?;
    println!("{}", html);
    Ok(())
}

fn render() -> Result<String, MyError> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}

#[derive(Debug)]
enum MyError {
    EnvironmentVariableNotFound,
    IOError(std::io::Error),
}

impl From<std::env::VarError> for MyError {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvironmentVariableNotFound
    }
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::EnvironmentVariableNotFound => write!(f, "Environment variable not found"),
            MyError::IOError(err) => write!(f, "IO Error: {}", err.to_string()),
        }
    }
}