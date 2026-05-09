use nix::unistd;
use std::process::Command;
use std::{env, io, process};

pub fn root_check() {
    let uid = unistd::getuid();
    let current_exe = env::current_exe().unwrap();
    if !uid.is_root() {
        Command::new("sudo").arg(current_exe).status().unwrap();
        process::exit(0);
    }
}

pub struct Iptables {
    table: Option<String>,
    flag: String,
    chain: String,
    conditions: Vec<String>,
    target: String,
    target_opt: Vec<String>,
}

impl Iptables {
    pub fn new(
        table: Option<String>,
        flag: String,
        chain: String,
        conditions: Vec<String>,
        target: String,
        target_opt: Vec<String>,
    ) -> Self {
        Self {
            table,
            flag,
            chain,
            conditions,
            target,
            target_opt,
        }
    }

    // sudo iptables -t filter -I OUTPUT -p tcp --dport 443 -j NFQUEUE --queue-num 0

    fn build_args(&self, flag: &str) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        if let Some(t) = &self.table {
            args.push("-t".to_string());
            args.push(t.clone());
        }
        args.push(flag.to_string());
        args.push(self.chain.clone());
        args.extend(self.conditions.clone());
        args.push("-j".to_string());
        args.push(self.target.clone());
        args.extend(self.target_opt.clone());
        args
    }

    pub fn apply(&self) -> Result<(), io::Error> {
        Command::new("iptables")
            .args(self.build_args(&self.flag))
            .status()?;
        Ok(())
    }

    // sudo iptables -D OUTPUT -p tcp --dport 443 -j NFQUEUE --queue-num 0

    pub fn iptables_remove(&self) -> Result<(), io::Error> {
        Command::new("iptables")
            .args(self.build_args("-D"))
            .status()?;
        Ok(())
    }
}

impl Drop for Iptables {
    fn drop(&mut self) {
        if let Err(e) = self.iptables_remove() {
            eprintln!("Cannot drop the previous process: {e}");
        }
    }
}
