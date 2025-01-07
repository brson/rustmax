use rmx::prelude::*;
use rmx::serde::{Serialize, Deserialize};
use rmx::xshell;
use rmx::json5;

#[derive(Serialize, Deserialize)]
struct Books {
    books: Vec<Book>,
}

#[derive(Serialize, Deserialize)]
struct Book {
    slug: String,
    name: String,
    repo: String,
}

pub fn build_library() -> AnyResult<()> {
    build_books(&load()?.books)
}

pub fn build_one_book(slug: &str) -> AnyResult<()> {
    let books = load();
    let book: Vec<Book> = load()?.books
        .into_iter()
        .filter(|b| b.slug == slug)
        .collect();
    if book.is_empty() {
        return Err(anyhow!("unknown book '{slug}'"));
    }
    build_books(&book)
}

fn build_books(books: &[Book]) -> AnyResult<()> {
    let procs = [
        get_repo,
        build_book,
    ];

    for proc in procs {
        for book in books {
            proc(book)?;
        }
    }

    Ok(())
}

fn get_repo(book: &Book) -> AnyResult<()> {
    todo!()
}

fn build_book(book: &Book) -> AnyResult<()> {
    todo!()
}

fn load() -> AnyResult<Books> {
    let path = "src/books.json5";
    let json = std::fs::read_to_string(path)?;
    Ok(json5::from_str(&json)?)
}
