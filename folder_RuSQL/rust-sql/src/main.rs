use mysql::*;
use mysql::prelude::*;
use std::io::{self, Write};

fn main() -> mysql::Result<()> {
    // Update with your MySQL connection details
    let url = "mysql://your_username:your_password@localhost:3306/testing";

    // Connect to the database
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // Create the books table
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS books (
            id INT AUTO_INCREMENT PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            author VARCHAR(255) NOT NULL,
            year INT NOT NULL
        )"
    )?;

    // Insert some entries into the books table
    let books = vec![
        ("The Catcher in the Rye", "J.D. Salinger", 1951),
        ("To Kill a Mockingbird", "Harper Lee", 1960),
        ("1984", "George Orwell", 1949),
    ];

    for book in books {
        conn.exec_drop(
            r"INSERT INTO books (title, author, year) VALUES (:title, :author, :year)",
            params! {
                "title" => book.0,
                "author" => book.1,
                "year" => book.2,
            },
        )?;
    }

    println!("Books table created and entries added.");

    // Call list_books to display all books
    list_books(&mut conn)?;

    // Call add_book to add a new book
    add_book(&mut conn)?;

    // List books again to show the newly added book
    list_books(&mut conn)?;

    Ok(())
}

// Function to list all books
fn list_books(conn: &mut PooledConn) -> mysql::Result<()> {
    let books: Vec<(i32, String, String, i32)> = conn.query(
        r"SELECT id, title, author, year FROM books"
    )?;

    println!("\nList of Books:");
    for book in books {
        println!("ID: {}, Title: {}, Author: {}, Year: {}", book.0, book.1, book.2, book.3);
    }

    Ok(())
}

// Function to add a new book
fn add_book(conn: &mut PooledConn) -> mysql::Result<()> {
    let mut title = String::new();
    let mut author = String::new();
    let mut year = String::new();

    print!("\nEnter the title of the book: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    print!("Enter the author of the book: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut author).unwrap();
    let author = author.trim();

    print!("Enter the year of publication: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    let year: i32 = year.trim().parse().expect("Please enter a valid year");

    conn.exec_drop(
        r"INSERT INTO books (title, author, year) VALUES (:title, :author, :year)",
        params! {
            "title" => title,
            "author" => author,
            "year" => year,
        },
    )?;

    println!("Book added successfully.");

    Ok(())
}
