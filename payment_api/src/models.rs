#[derive(Queryable)]
pub struct Transfer {
    pub id: i32,
    pub amount: String,
    pub currency: String,
    pub to_name: String,
    pub to_number: String,
    pub to_email: String,
    pub complete: bool,
}

