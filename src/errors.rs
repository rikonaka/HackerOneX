use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TargetAddrParseError {
    msg: String,
}

impl TargetAddrParseError {
    pub fn new(msg: &str) -> TargetAddrParseError {
        let msg = format!("wrong target address format: {}", msg);
        TargetAddrParseError { msg }
    }
}

impl fmt::Display for TargetAddrParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for TargetAddrParseError {
    fn description(&self) -> &str {
        &self.msg
    }
}

#[derive(Debug)]
pub struct InsufficientPortsError {
    msg: String,
}

impl InsufficientPortsError {
    pub fn new(msg: &str) -> InsufficientPortsError {
        let msg = format!("insufficient ports to support probing continue: {}", msg);
        InsufficientPortsError { msg }
    }
}

impl fmt::Display for InsufficientPortsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for InsufficientPortsError {
    fn description(&self) -> &str {
        &self.msg
    }
}
