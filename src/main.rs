mod parser;
mod pkt_control;
use std::io;

fn main() -> io::Result<()> {
    pkt_control::start_control()?;
    Ok(())
}
