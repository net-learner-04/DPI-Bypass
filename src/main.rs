mod config;
mod parser;
mod pkt_control;
use config::Iptables;
use std::io;

fn main() -> io::Result<()> {
    config::root_check();

    let rule = Iptables::new(
        Some("filter".to_string()),
        "-I".to_string(),
        "OUTPUT".to_string(),
        vec![
            "-p".to_string(),
            "tcp".to_string(),
            "--dport".to_string(),
            "443".to_string(),
        ],
        "NFQUEUE".to_string(),
        vec!["--queue-num".to_string(), "0".to_string()],
    );

    rule.apply()?;

    pkt_control::start_control()?;

    Ok(())
}
