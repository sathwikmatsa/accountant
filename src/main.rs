use accountant::*;
use diesel::prelude::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use models::*;

#[macro_use]
extern crate clap;

fn main() -> Result<(), ExitFailure> {
    use clap::App;
    let yml = load_yaml!("cli.yml");
    let matches = App::from(yml).get_matches();

    let conn = establish_connection()?;

    if let Some(matches) = matches.subcommand_matches("friend") {
        if let Some(_) = matches.subcommand_matches("list") {
            use schema::friend_t::dsl::*;
            let friends = friend_t
                .load::<Friend>(&conn)
                .with_context(|_| "Error loading friends")?;

            println!("Displaying friends list");
            for friend in friends {
                println!("{} {} {}", friend.name, friend.upi_id, friend.phone);
            }
        }
    }
    Ok(())
}
