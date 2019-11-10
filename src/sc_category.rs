use super::models::Category;
use super::models::NewCategory;
use clap::ArgMatches;
use diesel::prelude::*;
use exitfailure::ExitFailure;
use failure::ResultExt;

fn create_category(conn: &SqliteConnection, category_name: &str) -> Result<usize, ExitFailure> {
    use super::schema::category_t::dsl::*;

    let new_category = NewCategory {
        kind: category_name,
    };

    let rows_inserted = diesel::insert_into(category_t)
        .values(&new_category)
        .execute(conn)
        .with_context(|_| "Error saving new category")?;

    Ok(rows_inserted)
}

fn delete_category(conn: &SqliteConnection, category_name: &str) -> Result<usize, ExitFailure> {
    use super::schema::category_t::dsl::*;

    let rows_deleted = diesel::delete(category_t.filter(kind.eq(category_name)))
        .execute(conn)
        .with_context(|_| "Error deleting category")?;

    if rows_deleted == 0 {
        let error = Err(failure::err_msg(format!(
            "No category with name: {}",
            category_name
        )));
        let _ = Ok::<(), ExitFailure>(error.context("Delete operation failed")?);
    }

    Ok(rows_deleted)
}

pub fn eval(matches: &ArgMatches, conn: SqliteConnection) -> Result<(), ExitFailure> {
    use super::schema::category_t::dsl::*;

    if let Some(_) = matches.subcommand_matches("list") {
        let categories = category_t
            .load::<Category>(&conn)
            .with_context(|_| "Error loading categories")?;

        if categories.len() == 0 {
            eprintln!("No categories specified as of now.");
            eprintln!("Use `accountant category new_category` to add new category.");
            return Ok(());
        }

        println!("displaying category list:");
        for category in categories {
            println!("{}", category.kind);
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        let rows_deleted = delete_category(&conn, matches.value_of("kind").unwrap())?;
        if rows_deleted != 0 {
            println!("Successfully deleted from database.");
        }
    } else {
        let rows_added = create_category(&conn, matches.value_of("kind").unwrap())?;

        if rows_added != 0 {
            println!("Successfully added to database.");
        }
    }

    Ok(())
}
