use super::schema::friend_t;
use super::schema::category_t;

#[derive(Queryable)]
pub struct Friend {
    pub name: String,
    pub upi_id: String,
    pub phone: String,
}

#[derive(Insertable)]
#[table_name = "friend_t"]
pub struct NewFriend<'a> {
    pub name: &'a str,
    pub upi_id: &'a str,
    pub phone: &'a str,
}

#[derive(Queryable)]
pub struct Category {
    pub kind: String,
}

#[derive(Insertable)]
#[table_name = "category_t"]
pub struct NewCategory<'a> {
    pub kind: &'a str
}
