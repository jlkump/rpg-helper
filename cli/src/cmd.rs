use std::fmt::Display;

pub mod default;
pub mod data;

#[derive(Clone, Debug)]
pub enum CmdContext
{
    Default,
    Data(data::CtxData),
}

impl CmdContext
{
    pub fn to_prompt(&self) -> String
    {
        match self
        {
            CmdContext::Default => "[Default] >> ".to_string(),
            CmdContext::Data(ctx_data) => ctx_data.to_prompt(),
        }
    }
}

impl Display for CmdContext
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            CmdContext::Default =>
            {
                write!(f, "Default")
            },
            CmdContext::Data(ctx_data) =>
            {
                if ctx_data.open.is_some()
                {
                    write!(f, "Data - Open")
                }
                else
                {
                    write!(f, "Data")
                }
            },
        }
    }
}