use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum DisplayError
{
    InvalidColorFormat(String)
}