#[derive(Debug)]
pub enum InsertionError<T> {
    Conflicting(Vec<T>)
}

#[derive(Debug)]
pub enum FieldError {
    Mismatch(String, String),
    Nonexistant(String)
}

#[derive(Debug)]
pub enum DefinitionError<T> {
    Redef(T)
}

pub enum Error {
    
}