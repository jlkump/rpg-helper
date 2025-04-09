pub mod imp;

pub enum CachedData<T>
{
    Dirty, // Cache value is invalid, we need to request an update
    Valid(T), // Cache value is valid
}

pub enum NetworkError
{
    
}