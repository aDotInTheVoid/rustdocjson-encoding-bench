use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let s = fs::read_to_string("./corpus/quiche.json")?;
    let krate: rustdoc_types::Crate = serde_json::from_str(&s)?;
    dbg!(krate.format_version);

    Ok(())
}
