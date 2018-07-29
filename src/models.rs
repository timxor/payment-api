use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub email: String,
    pub public_key: String,
    pub private_key: String,
    pub eth_address: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub user_name: &'a str,
    pub email: &'a str,
    pub public_key: &'a str,
    pub private_key: &'a str,
    pub eth_address: &'a str
}
