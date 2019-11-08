use super::models::NewFriend;
use diesel::prelude::*;
use exitfailure::ExitFailure;
use failure::ResultExt;

pub fn create_friend(
    conn: &SqliteConnection,
    name: &str,
    upi_id: &str,
    phone: &str,
) -> Result<usize, ExitFailure> {
    use super::schema::friend_t;

    let new_friend = NewFriend {
        name,
        upi_id,
        phone,
    };

    let rows_inserted = diesel::insert_into(friend_t::table)
        .values(&new_friend)
        .execute(conn)
        .with_context(|_| "Error saving new friend contact")?;

    Ok(rows_inserted)
}
