# How to convert a Enum to DB row string value using Diesel? 

# DB Entity Scheme
Lets take, for example, this DB Entity Scheme:
```rust
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
    pub isUrgent: bool,
    pub isImportant: bool,
    pub deadline: NaiveDateTime,
    pub deleted: bool
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
    pub isUrgent: bool,
    pub isImportant: bool,
    pub deadline: NaiveDateTime,
    pub deleted: bool
}

```

## Implement the converters
You need the following enum and Diesel converters (to_sql, from_sql). The `set_value` in the function allows a direct setting of the "return" value. 

Two important aspects regarding e.g. `ToSql<Text, diesel::sqlite::Sqlite>`! 
- `Text` is a diesel type (=`diesel::sql_types::Text`, those can be found [here](https://docs.rs/diesel/latest/diesel/sql_types/index.html))
- the DB Type must be defined next to it e.g. `diesel::sqlite::Sqlite` (works for PG and MySQL the same way)

The DB types are:
- `diesel::sqlite::Sqlite` [Sqlite DB Struct](https://docs.rs/diesel/latest/diesel/sqlite/index.html)
- `diesel::pg::Pg` [PG DB Struct](https://docs.rs/diesel/latest/diesel/pg/struct.Pg.html)
- `diesel::mysql::Mysql` [MySQL DB Struct](https://docs.rs/diesel/latest/diesel/mysql/struct.Mysql.html)

To make the converting easier the `Strum` Traits are used. Strum needs to be installed additionally. Either by using cargo add or editing the `cargo.toml`

**WARN:** Do not forget to add "sqlite" to the features array in `cargo.toml`!


```rust
use strum_macros::{Display, EnumString};
use crate::models::deserialize::FromSqlRow;

#[repr(i32)]
#[derive(SqlType, Debug, PartialEq, FromSqlRow, AsExpression, Display, Copy, Clone, EnumString)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum TaskStatus {
    New,
    OnHold,
    InProgress,
    Done
}

impl ToSql<Text, diesel::sqlite::Sqlite> for TaskStatus
where
    str: ToSql<Text, diesel::sqlite::Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for TaskStatus {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        Ok(<String as FromSql<Text, Sqlite>>::from_sql(bytes).map(|s| TaskStatus::from_str(&s).unwrap())?)
    }
}
```

## Proof that it works
![working_show_tasks](https://github.com/diesel-rs/diesel/assets/37943746/2498bf80-b731-45f2-b21f-86ae8febc066)
![working_show_tasks_2](https://github.com/diesel-rs/diesel/assets/37943746/3d517f8e-0e26-4c98-98a8-8d8c4500ef4c)

## Related issue
- [https://github.com/diesel-rs/diesel/discussions/3742](https://github.com/diesel-rs/diesel/discussions/3742)
