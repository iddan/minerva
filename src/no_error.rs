// Just to make error in stream satisfied

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NoError;

impl fmt::Display for NoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Error for NoError {}
