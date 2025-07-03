use std::collections::HashMap;

use crate::api::data::{tag::Tag, DataType};

pub struct UserDefinedData
{
    name: Tag,
    structure: HashMap<String, DataType>,
}

pub struct UserDefinedDataInst
{
    type_name: Tag,
}