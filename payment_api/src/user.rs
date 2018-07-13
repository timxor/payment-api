use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use schema::users;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl Hero {
    pub fn create(hero: Hero, connection: &MysqlConnection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new hero");

        heroes::table.order(heroes::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Hero> {
        heroes::table.order(heroes::id.asc()).load::<Hero>(connection).unwrap()
    }

    pub fn update(id: i32, hero: Hero, connection: &MysqlConnection) -> bool {
        diesel::update(heroes::table.find(id)).set(&hero).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(heroes::table.find(id)).execute(connection).is_ok()
    }
}
