use super::models::Friend;
use super::models::NewFriend;
use clap::ArgMatches;
use diesel::prelude::*;
use exitfailure::ExitFailure;
use failure::ResultExt;

fn create_friend(
    conn: &SqliteConnection,
    naam: &str,
    upi: &str,
    ph: &str,
) -> Result<usize, ExitFailure> {
    use super::schema::friend_t::dsl::*;

    let new_friend = NewFriend {
        name: naam,
        upi_id: upi,
        phone: ph,
    };

    let rows_inserted = diesel::insert_into(friend_t)
        .values(&new_friend)
        .execute(conn)
        .with_context(|_| "Error saving new friend contact")?;

    Ok(rows_inserted)
}

fn delete_friend(conn: &SqliteConnection, naam: &str) -> Result<usize, ExitFailure> {
    use super::schema::friend_t::dsl::*;

    let rows_deleted = diesel::delete(friend_t.filter(name.eq(naam)))
        .execute(conn)
        .with_context(|_| "Error deleting friend")?;

    if rows_deleted == 0 {
        let error = Err(failure::err_msg(format!("No friend with name: {}", naam)));
        let _ = Ok::<(), ExitFailure>(error.context("Delete operation failed")?);
    }

    Ok(rows_deleted)
}

pub fn eval(matches: &ArgMatches, conn: SqliteConnection) -> Result<(), ExitFailure> {
    use super::schema::friend_t::dsl::*;

    if let Some(_) = matches.subcommand_matches("list") {
        let friends = friend_t
            .load::<Friend>(&conn)
            .with_context(|_| "Error loading friends")?;

        println!("displaying friends list:");
        for friend in friends {
            println!("{} {} {}", friend.name, friend.upi_id, friend.phone);
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        let rows_deleted = delete_friend(&conn, matches.value_of("name").unwrap())?;
        if rows_deleted != 0 {
            println!("Successfully deleted from database.");
        }
    } else if let Some(matches) = matches.subcommand_matches("find") {
        let pattern = matches.value_of("pattern").unwrap();
        let friends = friend_t
            .load::<Friend>(&conn)
            .with_context(|_| "Error loading friends")?;

        let results = friends.iter().filter(|&x| x.name.contains(&pattern));

        println!("search results:");
        for friend in results {
            println!("{} {} {}", friend.name, friend.upi_id, friend.phone);
        }
    } else {
        let rows_added = create_friend(
            &conn,
            matches.value_of("name").unwrap(),
            matches.value_of("upi_id").unwrap(),
            matches.value_of("phone").unwrap(),
        )?;

        if rows_added != 0 {
            println!("Successfully added to database.");
        }
    }

    Ok(())
}
