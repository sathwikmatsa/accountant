use accountant::*;
use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    let conn = establish_connection()?;
    reg::create_friend(&conn,
                       "krupa",
                       "kRUPA@oksbi",
                       "9000000000")?;

    reg::create_friend(&conn,
                       "tofu",
                       "tofraz@googlepay",
                       "9111111111")?;

    Ok(())
}
