use anyhow::Result;

pub fn run() -> Result<()> {
    println!("Claude Bell v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
