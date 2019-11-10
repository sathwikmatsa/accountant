use super::models::Expense;
use super::models::NewExpense;
use clap::ArgMatches;
use diesel::prelude::*;
use exitfailure::ExitFailure;
use failure::ResultExt;

fn create_expense(
    conn: &SqliteConnection,
    price: f32,
    desc: &str,
    c: &str,
    t: &str,
) -> Result<usize, ExitFailure> {
    use super::schema::expense_t::dsl::*;

    let new_expense = NewExpense {
        cost: price,
        description: desc,
        category: c,
        tags: t,
    };

    let rows_inserted = diesel::insert_into(expense_t)
        .values(&new_expense)
        .execute(conn)
        .with_context(|_| "Error saving new expense")?;

    Ok(rows_inserted)
}

pub fn list(matches: &ArgMatches, conn: SqliteConnection) -> Result<(), ExitFailure> {
    use super::schema::expense_t::dsl::*;

    let expenses = expense_t
        .load::<Expense>(&conn)
        .with_context(|_| "Error loading expenses")?;

    if expenses.len() == 0 {
        eprintln!("No expenses registered as of now.");
        eprintln!("Use `accountant <cost> <description>` to add new expense.");
        return Ok(());
    }

    let mut start = 0;
    let mut end = expenses.len();

    if let Some(first) = matches.value_of("first") {
        let n: usize = first
            .parse::<usize>()
            .with_context(|_| "Value provided for --first is not a valid number")?;

        if n < expenses.len() {
            end = n;
            println!(
                "displaying first {} expenses: [cost, description, category, tags, ts]",
                n
            );
        } else {
            println!("displaying all expenses: [cost, description, category, tags, ts]");
        }
    } else if let Some(last) = matches.value_of("last") {
        let n: usize = last
            .parse::<usize>()
            .with_context(|_| "Value provided for --last is not a valid number")?;

        if n < expenses.len() {
            start = expenses.len() - n;
            println!(
                "displaying latest {} expenses: [cost, description, category, tags, ts]",
                n
            );
        } else {
            println!("displaying all expenses: [cost, description, category, tags, ts]");
        }
    } else {
        println!("displaying all expenses: [cost, description, category, tags, ts]");
    }
    for i in start..end {
        println!(
            "{} {} {} {} {}",
            expenses[i].cost,
            expenses[i].description,
            expenses[i].category,
            expenses[i].tags,
            expenses[i].ts,
        );
    }

    Ok(())
}

pub fn add(matches: &ArgMatches, conn: SqliteConnection) -> Result<(), ExitFailure> {
    let category_arg = matches.value_of("category").unwrap_or("other");
    let mut category_registered = false;

    {
        use super::models::Category;
        use super::schema::category_t::dsl::*;
        let categories: String = category_t
            .load::<Category>(&conn)
            .with_context(|_| "Error loading categories")?
            .iter()
            .fold("".to_string(), |acc, x| acc + &x.kind);

        if categories.contains(category_arg) || category_arg == "other" {
            category_registered = true;
        }
    }

    if !category_registered {
        let error = Err(failure::err_msg(format!(
            "No category with name: {}",
            category_arg
        )));
        let _ = Ok::<(), ExitFailure>(error.context("Registering expense failed")?);
    }

    let price: f32 = matches
        .value_of("cost")
        .unwrap()
        .parse::<f32>()
        .with_context(|_| "Cost value is not a valid number")?;

    let rows_added = create_expense(
        &conn,
        price,
        matches.value_of("description").unwrap(),
        category_arg,
        matches.value_of("tags").unwrap_or("unspecified"),
    )?;

    if rows_added != 0 {
        println!("Successfully registered the expense in database.");
    }

    Ok(())
}
