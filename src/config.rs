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

    //sudo iptables -t filter -I OUTPUT -p tcp --dport 443 -j NFQUEUE --queue-num 0

    pub fn apply(&self) -> Result<(), io::Error> {
        let mut args: Vec<String> = Vec::new();

        if let Some(t) = &self.table {
            args.push("-t".to_string());
            args.push(t.clone());
        }
        args.push(self.flag.clone());
        args.push(self.chain.clone());
        for condition in &self.conditions {
            args.push(condition.clone());
        }
        args.push("-j".to_string());
        args.push(self.target.clone());
        for opt in &self.target_opt {
            args.push(opt.clone());
        }

        Command::new("iptables").args(args).status()?;

        Ok(())
    }

    //sudo iptables -D OUTPUT 1

    pub fn iptables_remove(&self) -> Result<(), io::Error> {
        let nfq_number = Self::get_nfqueue();

        match nfq_number {
            Some(n) => {
                let n_str = n.to_string();
                Command::new("iptables")
                    .args(["-D", "OUTPUT", &n_str])
                    .status()?;
                Ok(())
            }
            None => Err(io::Error::other("NFQUEUE does not exist")),
        }
    }

    //sudo iptables -L OUTPUT --line-numbers -n

    fn get_nfqueue() -> Option<u32> {
        let nfqueue = Command::new("sudo")
            .arg("iptables")
            .args(["-L", "OUTPUT", "--line-numbers", "-n"])
            .output()
            .ok()?;

        let nfqueue = String::from_utf8_lossy(&nfqueue.stdout);

        for line in nfqueue.lines() {
            if line.contains("NFQUEUE") {
                let nfq_num = line.split_whitespace().next();

                match nfq_num {
                    Some(n) => return n.parse::<u32>().ok(),
                    None => return None,
                }
            }
        }
        None
    }
}

impl Drop for Iptables {
    fn drop(&mut self) {
        if let Err(e) = self.iptables_remove() {
            eprintln!("Cannot drop the previous process: {e}");
        }
    }
}
