use std::error::Error;

mod file;
mod one;
fn main() -> Result<(), Box<dyn Error>> {
    let sum = one::run(true)?;
    println!("{}", sum);
    Ok(())
}
