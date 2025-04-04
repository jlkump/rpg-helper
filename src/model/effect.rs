


pub struct Effect<T>
{
    pub name: String,
    pub action: Action<T>,
}

pub struct Res
{

}

pub struct Action<T>
{
    f: dyn Fn(T) -> Res,
}