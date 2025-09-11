use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Icon
{
    Delete,
    Add,
    Edit,
    Help,
    Search,
    Clear,
    Reset,
}