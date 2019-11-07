use super::schema::friend_t;

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
