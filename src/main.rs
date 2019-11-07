use accountant::*;
use diesel::prelude::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use models::*;

fn main() -> Result<(), ExitFailure> {
    use schema::friend_t::dsl::*;

    let conn = establish_connection()?;
    let friends = friend_t
        .load::<Friend>(&conn)
        .with_context(|_| "Error loading friends")?;

    println!("Displaying friends list");
    for friend in friends {
        println!("{} {} {}", friend.name, friend.upi_id, friend.phone);
    }

    Ok(())
}
