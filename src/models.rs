use chrono::naive::NaiveDateTime;
use super::schema::friend_t;
use super::schema::category_t;
use super::schema::expense_t;

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


#[derive(Queryable)]
pub struct Expense {
    pub id: i32,
    pub cost: f32,
    pub description: String,
    pub category: String,
    pub tags: String,
    pub ts: NaiveDateTime
}

#[derive(Insertable)]
#[table_name = "expense_t"]
pub struct NewExpense<'a> {
    pub cost: f32,
    pub description: &'a str,
    pub category: &'a str,
    pub tags: &'a str
}
