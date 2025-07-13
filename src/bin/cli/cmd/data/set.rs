use std::process::exit;

use colored::{ColoredString, Colorize};
use rpg_helper::api::data::{effect::Effect, equation::Equation, tag::Tag};

use crate::cmd::{data::CtxSubmode, CmdContext};


#[derive(Clone, Debug)]
pub enum CtxData
{
    Attribute(AtrCtxData),
    Conditional(),
    Equation(EqCtxData),
    Modifier(),
    Tag(TagCtxData),
    Text(),
}

impl CtxData
{
    pub fn new_attribute() -> Self
    {
        CtxData::Attribute(AtrCtxData::RequestAtrName)
    }

    pub fn new_conditional() -> Self
    {
        CtxData::Conditional()
    }

    pub fn new_equation() -> Self
    {
        CtxData::Equation(EqCtxData::RequestEqName)
    }

    pub fn new_modifier() -> Self
    {
        CtxData::Modifier()
    }

    pub fn new_tag() -> Self
    {
        CtxData::Tag(TagCtxData::RequestTagName)
    }

    pub fn new_text() -> Self
    {
        CtxData::Text()
    }

    pub fn to_prompt(&self) -> String
    {
        match self
        {
            CtxData::Attribute(a) =>
            {
                match a
                {
                    AtrCtxData::RequestAtrName => "Please enter an attribute name: ".to_string(),
                    AtrCtxData::RequestAtrValue(tag) => format!("Please enter a value for attribute \"{}\": ", tag),
                    AtrCtxData::ConfirmAtr(tag, val) => format!("Confirm attribute \"{}: {}\" [y/n]: ", tag, val),
                }
            },
            CtxData::Conditional() => todo!(),
            CtxData::Equation(e) =>
            {
                match e
                {
                    EqCtxData::RequestEqName => "Please enter an equation name: ".to_string(),
                    EqCtxData::RequestEqValue(tag) => format!("Please enter an equation for \"{}\": ", tag),
                    EqCtxData::ConfirmEq(tag, eq) => format!("Confirm equation \"{}: {}\" [y/n]: ", tag, eq),
                }
            },
            CtxData::Modifier() => todo!(),
            CtxData::Tag(t) =>
            {
                match t
                {
                    TagCtxData::RequestTagName => "Please enter a tag name: ".to_string(),
                    TagCtxData::ConfirmTagName(n) => format!("Confirm tag name \"{}\" [y/n]: ", n),
                }
            },
            CtxData::Text() => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TagCtxData
{
    RequestTagName,
    ConfirmTagName(Tag),
}

#[derive(Clone, Debug)]
pub enum AtrCtxData
{
    RequestAtrName,
    RequestAtrValue(Tag),
    ConfirmAtr(Tag, f32),
}

#[derive(Clone, Debug)]
pub enum EqCtxData
{
    RequestEqName,
    RequestEqValue(Tag),
    ConfirmEq(Tag, String),
}

pub fn execute_command(s: &str, cmd_context: &mut CmdContext) -> Result<ColoredString, ColoredString> 
{
    match cmd_context
    {
        CmdContext::Default =>
        {
            error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
            exit(1)
        },
        CmdContext::Data(ctx_data) =>
        {
            match &ctx_data.ctx_submode
            {
                super::CtxSubmode::Default =>
                {
                    error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
                    exit(1)
                },
                super::CtxSubmode::Set(ctx_data) =>
                {
                    match &ctx_data
                    {
                        CtxData::Attribute(_) => execute_attribute(s, cmd_context),
                        CtxData::Conditional() => todo!(),
                        CtxData::Equation(_) => execute_equation(s, cmd_context),
                        CtxData::Modifier() => todo!(),
                        CtxData::Tag(_) => execute_tag(s, cmd_context),
                        CtxData::Text() => todo!(),
                    }
                },
            }
        },
    }
}

fn execute_attribute(s: &str, cmd_context: &mut CmdContext) -> Result<ColoredString, ColoredString>
{
    if let CmdContext::Data(data_ctx) = cmd_context
    {
        if let CtxSubmode::Set(ctx) = &mut data_ctx.ctx_submode
        {
            if let CtxData::Attribute(ctx) = ctx
            {
                match ctx.clone()
                {
                    AtrCtxData::RequestAtrName =>
                    {
                        match Tag::from_str(s)
                        {
                            Ok(tag) => 
                            {
                                info!("[Set - Atr] Created tag \"{}\"", tag);
                                *ctx = AtrCtxData::RequestAtrValue(tag.clone());
                                Ok(format!("Attribute name formatted: \"{}\"", tag).cyan())
                            },
                            Err(e) =>
                            {
                                error!("[Set - Atr] Parse error on input tag \"{}\":\n{:?}", s, e);
                                Err(format!("Could not parse given tag \"{}\":\n{:?}", s, e).red())
                            },
                        }
                    },
                    AtrCtxData::RequestAtrValue(t) =>
                    {
                        if let Ok(val) = s.trim().parse()
                        {
                            info!("[Set - Atr] Set value of \"{}: {}\"", t, val);
                            *ctx = AtrCtxData::ConfirmAtr(t.clone(), val);
                            Ok(format!("Attribute value formatted: \"{}\"", val).cyan())
                        }
                        else
                        {
                            warn!("[Set - Atr] Could not parse input as f32: \"{}\"", s);
                            Err(format!("Could not parse input \"{}\" as number", s).red())
                        }
                    },
                    AtrCtxData::ConfirmAtr(t, v) =>
                    {
                        let s = s.trim();
                        if s == "y" || s == "yes"
                        {
                            info!("[Set - Atr] Set attribute \"{}: {}\"", t, v);
                            data_ctx.open = match data_ctx.open.clone().unwrap().apply_effect(&Effect::SetAttribute(t.clone(), v))
                            {
                                Ok(c) => Some(c),
                                Err(e) =>
                                {
                                    error!("[Set - Atr] Could not set attribute \"{}: {}\":\n{:?}", t, v, e);
                                    return Err(format!("Could not apply attribute to data set \"{}: {}\":\n{:?}", t, v, e).red())
                                },
                            };
                            data_ctx.ctx_submode = CtxSubmode::Default;
                            Ok(format!("Set attribute \"{}: {}\"", t, v).cyan())
                        }
                        else if s == "n" || s == "no"
                        {
                            info!("[Set - Atr] Discard attribute \"{}: {}\"", t, v);
                            *ctx = AtrCtxData::RequestAtrName;
                            Ok(format!("Discarding attribute: \"{}: {}\"", t, v).cyan())
                        }
                        else
                        {
                            warn!("[Set - Atr] Confirming attribute, unrecognized response \"{}\"", s);
                            Err(format!("Unrecognized response \"{}\", please input \"yes\" or \"no\"", s).red())
                        }
                    },
                }
            }
            else
            {
                error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
                exit(1)
            }
        }
        else
        {
            error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
            exit(1)
        }
    }
    else
    {
        error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
        exit(1)
    }
}

fn execute_equation(s: &str, cmd_context: &mut CmdContext) -> Result<ColoredString, ColoredString>
{
    if let CmdContext::Data(data_ctx) = cmd_context
    {
        if let CtxSubmode::Set(ctx) = &mut data_ctx.ctx_submode
        {
            if let CtxData::Equation(ctx) = ctx
            {
                match ctx.clone()
                {
                    EqCtxData::RequestEqName =>
                    {
                        match Tag::from_str(s)
                        {
                            Ok(tag) => 
                            {
                                info!("[Set - Eq] Created tag \"{}\"", tag);
                                *ctx = EqCtxData::RequestEqValue(tag.clone());
                                Ok(format!("Equation name formatted: \"{}\"", tag).cyan())
                            },
                            Err(e) =>
                            {
                                error!("[Set - Eq] Parse error on input tag \"{}\":\n{:?}", s, e);
                                Err(format!("Could not parse given tag \"{}\":\n{:?}", s, e).red())
                            },
                        }
                    },
                    EqCtxData::RequestEqValue(tag) =>
                    {
                        if let Err(e) = Equation::new(tag.clone(), s)
                        {
                            error!("[Set - Eq] Error on equation creation \"{}\":\n{:?}", s, e);
                            Err(format!("Got error from input equation \"{}\":\n{:?}", s, e).red())
                        }
                        else
                        {
                            info!("[Set - Eq] Equation formatted \"{}\"", s);
                            *ctx = EqCtxData::ConfirmEq(tag.clone(), s.to_string());
                            Ok(format!("Equation value formatted: \"{}\"", s).cyan())
                        }
                    },
                    EqCtxData::ConfirmEq(tag, eq) =>
                    {
                        let s = s.trim();
                        if s == "y" || s == "yes"
                        {
                            info!("[Set - Eq] Set equationn \"{}: {}\"", tag, eq);
                            data_ctx.open = match data_ctx.open.clone().unwrap().apply_effect(&Effect::SetEquation(Equation::new(tag.clone(), &eq).unwrap()))
                            {
                                Ok(c) => Some(c),
                                Err(e) =>
                                {
                                    error!("[Set - Eq] Could not set equation \"{}: {}\":\n{:?}", tag, eq, e);
                                    return Err(format!("Could not apply equation to data set \"{}: {}\":\n{:?}", tag, eq, e).red())
                                },
                            };
                            data_ctx.ctx_submode = CtxSubmode::Default;
                            Ok(format!("Set equation \"{}: {}\"", tag, eq).cyan())
                        }
                        else if s == "n" || s == "no"
                        {
                            info!("[Set - Eq] Discard equation \"{}: {}\"", tag, eq);
                            *ctx = EqCtxData::RequestEqName;
                            Ok(format!("Discarding equation: \"{}: {}\"", tag, eq).cyan())
                        }
                        else
                        {
                            warn!("[Set - Eq] Confirming equation, unrecognized response \"{}\"", s);
                            Err(format!("Unrecognized response \"{}\", please input \"yes\" or \"no\"", s).red())
                        }
                    },
                }
            }
            else
            {
                error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
                exit(1)
            }
        }
        else
        {
            error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
            exit(1)
        }
    }
    else
    {
        error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
        exit(1)
    }
}


fn execute_tag(s: &str, cmd_context: &mut CmdContext) -> Result<ColoredString, ColoredString>
{
    if let CmdContext::Data(data_ctx) = cmd_context
    {
        if let CtxSubmode::Set(ctx) = &mut data_ctx.ctx_submode
        {
            if let CtxData::Tag(ctx) = ctx
            {
                match ctx.clone()
                {
                    TagCtxData::RequestTagName =>
                    {
                        match Tag::from_str(s)
                        {
                            Ok(tag) => 
                            {
                                info!("[Set - Tag] Created tag \"{}\"", tag);
                                *ctx = TagCtxData::ConfirmTagName(tag.clone());
                                Ok(format!("Tag formatted: \"{}\"", tag).cyan())
                            },
                            Err(e) =>
                            {
                                error!("[Set - Tag] Parse error on input tag \"{}\":\n{:?}", s, e);
                                Err(format!("Could not parse given tag \"{}\":\n{:?}", s, e).red())
                            },
                        }
                    },
                    TagCtxData::ConfirmTagName(t) =>
                    {
                        let s = s.trim();
                        if s == "y" || s == "yes"
                        {
                            info!("[Set - Tag] Set tag \"{}\"", t);
                            data_ctx.open = match data_ctx.open.clone().unwrap().apply_effect(&Effect::AddStateTag(t.clone()))
                            {
                                Ok(c) => Some(c),
                                Err(e) =>
                                {
                                    error!("[Set - Tag] Could not set tag \"{}\":\n{:?}", t, e);
                                    return Err(format!("Could not apply tag to data set \"{}\":\n{:?}", t, e).red())
                                },
                            };
                            data_ctx.ctx_submode = CtxSubmode::Default;
                            Ok(format!("Created new tag: \"{}\"", t).cyan())
                        }
                        else if s == "n" || s == "no"
                        {
                            info!("[Set - Tag] Discard tag \"{}\"", t);
                            *ctx = TagCtxData::RequestTagName;
                            Ok(format!("Discarding tag: \"{}\"", t).cyan())
                        }
                        else
                        {
                            warn!("[{}] Confirming tag, unrecognized response \"{}\"", cmd_context, s);
                            Err(format!("Unrecognized response \"{}\", please input \"yes\" or \"no\"", s).red())
                        }
                    },
                }
            }
            else
            {
                error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
                exit(1)
            }
        }
        else
        {
            error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
            exit(1)
        }
    }
    else
    {
        error!("[Set] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
        exit(1)
    }
}