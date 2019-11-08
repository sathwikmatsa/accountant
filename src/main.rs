use accountant::*;
use exitfailure::ExitFailure;

#[macro_use]
extern crate clap;

fn main() -> Result<(), ExitFailure> {
    use clap::App;
    let yml = load_yaml!("cli.yml");
    let matches = App::from(yml).get_matches();

    let conn = establish_connection()?;

    if let Some(matches) = matches.subcommand_matches("friend") {
        sc_friend::eval(matches, conn)?;
    }
    Ok(())
}
