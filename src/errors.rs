use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TargetAddrParseError {
    addr: String,
}

impl TargetAddrParseError {
    pub fn new(msg: &str) -> TargetAddrParseError {
        TargetAddrParseError {
            addr: msg.to_string(),
        }
    }
}

impl fmt::Display for TargetAddrParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "wrong target address format: {}", self.addr)
    }
}

impl Error for TargetAddrParseError {
    fn description(&self) -> &str {
        &self.addr
    }
}
