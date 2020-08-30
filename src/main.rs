use octanol::engine::Octanol;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut octanol = Octanol::new();
    octanol.folder("templates/")?;
    octanol.run()?;
    Ok(())
}
