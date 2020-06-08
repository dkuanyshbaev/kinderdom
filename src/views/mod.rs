pub mod admin;
pub mod pages;

#[derive(Serialize)]
pub struct NoContext {}

#[derive(Serialize)]
pub struct ListContext<T> {
    items: Vec<T>,
}

#[derive(Serialize)]
pub struct ComplexContext<T, U> {
    item: U,
    items: Vec<T>,
}

#[derive(Serialize)]
pub struct IndexContext {
    payment_description: String,
}
