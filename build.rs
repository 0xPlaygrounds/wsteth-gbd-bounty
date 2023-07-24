use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Wsteth", "abi/wsteth.json")?
        .generate()?
        .write_to_file("src/abi/wsteth.rs")?;

    Ok(())
}