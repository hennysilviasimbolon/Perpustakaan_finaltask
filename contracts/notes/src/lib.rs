#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data Book (ubah dari Note)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Book {
    pub id: u64, 
    pub title: String,
    pub author: String,
    pub is_borrowed: bool,
}

// Storage key
const BOOK_DATA: Symbol = symbol_short!("BOOK_DATA");
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct LibraryContract;

#[contractimpl]
impl LibraryContract {

    // Ambil semua buku
    pub fn get_books(env: Env) -> Vec<Book> {
        env.storage()
            .instance()
            .get(&BOOK_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah buku
    pub fn add_book(env: Env, title: String, author: String) -> String {
        let mut books: Vec<Book> = env.storage()
            .instance()
            .get(&BOOK_DATA)
            .unwrap_or(Vec::new(&env));

        // pakai counter biar ID tidak random
        let mut counter: u64 = env.storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0);

        counter += 1;

        let book = Book {
            id: counter,
            title,
            author,
            is_borrowed: false,
        };

        books.push_back(book);

        env.storage().instance().set(&BOOK_DATA, &books);
        env.storage().instance().set(&COUNTER, &counter);

        String::from_str(&env, "Book berhasil ditambahkan")
    }

    // Hapus buku
    pub fn delete_book(env: Env, id: u64) -> String {
        let books: Vec<Book> = env.storage()
            .instance()
            .get(&BOOK_DATA)
            .unwrap_or(Vec::new(&env));

        let mut new_books = Vec::new(&env);
        let mut found = false;

        for book in books.iter() {
            if book.id != id {
                new_books.push_back(book.clone());
            } else {
                found = true;
            }
        }

        env.storage().instance().set(&BOOK_DATA, &new_books);

        if found {
            String::from_str(&env, "Book berhasil dihapus")
        } else {
            String::from_str(&env, "Book tidak ditemukan")
        }
    }

    // Pinjam buku
    pub fn borrow_book(env: Env, id: u64) -> String {
        let books: Vec<Book> = env.storage()
            .instance()
            .get(&BOOK_DATA)
            .unwrap_or(Vec::new(&env));

        let mut updated_books = Vec::new(&env);
        let mut found = false;

        for book in books.iter() {
            let mut b = book.clone();

            if b.id == id {
                if b.is_borrowed {
                    return String::from_str(&env, "Book already borrowed");
                }
                b.is_borrowed = true;
                found = true;
            }

            updated_books.push_back(b);
        }

        env.storage().instance().set(&BOOK_DATA, &updated_books);

        if found {
            String::from_str(&env, "Book berhasil dipinjam")
        } else {
            String::from_str(&env, "Book tidak ditemukan")
        }
    }
}

mod test;