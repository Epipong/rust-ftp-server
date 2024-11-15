use std::{io::Error, process::Command};

use regex::Regex;

pub fn get_local_ip() -> Result<String, Error> {
    let output = Command::new("ifconfig").output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let re = Regex::new(r"(wlan\d|eth\d):.*\n\s+inet\s([\d.]+)").expect("Error on regex");
    re.captures(&stdout)
        .and_then(|caps| caps.get(2).map(|m| m.as_str().to_string()))
        .ok_or(Error::new(std::io::ErrorKind::NotFound, "wlan or eth not found. Try to enable the WI-FI or to enable the personal hotspot"))
}
