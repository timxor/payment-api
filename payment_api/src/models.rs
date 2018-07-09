use schema::transfers;

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

#[derive(Insertable)]
#[table_name = "transfers"]
pub struct NewTransfer<'a> {
    pub amount: &'a str,
    pub currency: &'a str,
    pub to_name: &'a str,
    pub to_number: &'a str,
    pub to_email: &'a str,
    pub complete: &'a bool,
}
