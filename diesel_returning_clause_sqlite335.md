# How to use SQLite3.35+ and Diesel?
Sadly, the [Diesel Sqlite3 examples](https://github.com/diesel-rs/diesel/tree/master/examples/sqlite) does not contain any hint for the needed [Diesel PR 3004](https://github.com/diesel-rs/diesel/pull/3004). 
The PR adds a RUST Cargo feature for Diesel. 

Despite the importance of the cargo feature `returning_clauses_for_sqlite_3_35` it is not mentioned at all. The feature is required for the [Diesel returning clause](https://github.com/toxeus/diesel/blob/fda3410c1715a7cf57ae08f7969e297aa1762217/diesel_tests/tests/insert.rs#L214) in the SQLite context of Diesel. 

Put simply, SQLite 3.35 introduced the [SQLite returning clause](https://www.sqlite.org/lang_returning.html). The returning clause is used to get the new row/obj after a insert statement. Thus the missing returning clause affected Diesel until 2022. So until the PR and merge.

## Example: How does the returning clause look like in Diesel?
```rust
  pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str, published: bool) -> Post
  {
  
      let new_post = NewPost { title, body, published };
  
      insert_into(posts)
          .values(&new_post)
          .returning(Post::as_returning()) // <-- that little part here is the returning clause(!)
          .get_result(conn)
          .expect("Error saving new post")
  }
```

## Example: How does the `cargo.toml` look like?
```toml
diesel = {version="2.1.0", features = ["sqlite", "returning_clauses_for_sqlite_3_35"] }
```

