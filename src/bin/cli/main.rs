#[macro_use] extern crate log;
extern crate simplelog;

fn main() -> std::io::Result<()>
{
    println!(". is Alphanumeric: {}", '.'.is_alphanumeric());
    Ok(())
}