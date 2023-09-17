use clap::{self, ArgAction};
use clap::{command, Arg, Command};
use rusqlite;
use std::fs::File;
use std::io;

struct Budget {
    id: i32,
    name: String,
    remaining: f64,
}

struct Transaction {
    id: usize,
    budget_name: String,
    name: String,
    amount: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_conn()?;

    let matches = command!()
        .subcommand(Command::new("new").about("Initialised database"))
        .subcommand(
            Command::new("budget")
                .about("Add and remove budgets")
                .subcommand(
                    Command::new("add")
                        .arg(Arg::new("name").short('n').long("name").required(true))
                        .arg(Arg::new("amount").short('a').long("amount").required(true)),
                )
                .subcommand(Command::new("remove").arg(Arg::new("name").short('n').long("name"))),
        )
        .subcommand(Command::new("transaction").about("Add and remove transactions to budgets"))
        .get_matches();

    match matches.subcommand() {
        Some(("new", _)) => create_tables(conn)?,
        Some(("budget", sub_matches)) => match sub_matches.subcommand() {
            Some(("add", args)) => {
                new_budget(
                    args.get_one::<String>("name").unwrap(),
                    args.get_one::<f64>("amount").unwrap(),
                    conn,
                )?;
            }
            Some(("remove", args)) => {
                remove_budget(args.get_one::<String>("name").unwrap(), conn)?;
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}

fn get_conn() -> Result<rusqlite::Connection, Box<dyn std::error::Error>> {
    Ok(rusqlite::Connection::open("budget.db")?)
}

fn read_sql_file_to_string(location: &str) -> Result<String, std::io::Error> {
    let f = File::open(location)?;
    let reader = io::BufReader::new(f);
    io::read_to_string(reader)
}

fn create_tables(conn: rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>> {
    for file in vec!["sql/Budget.sql", "sql/Transaction.sql"] {
        let create_stmt = read_sql_file_to_string(file)?;
        conn.execute(&create_stmt, ())?;
    }
    Ok(())
}

fn new_budget(
    name: &str,
    amount: &f64,
    conn: rusqlite::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let budget = Budget {
        id: 0,
        name: name.to_string(),
        remaining: amount.clone(),
    };

    conn.execute(
        "INSERT INTO Budget (name, remaining) VALUES (?1, ?2)",
        (&budget.name, &budget.remaining),
    )?;

    Ok(())
}

fn remove_budget(name: &str, conn: rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(&format!("DELETE FROM Budget WHERE name = '{name}'"), ());
    Ok(())
}
