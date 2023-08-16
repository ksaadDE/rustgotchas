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
You need the following enum and Diesel converters (to_sql, from_sql). Two important aspects! Text is a diesel type, and the DB Type must be defined next to it. `set_value` allows a direct setting of the "return" value. However, additionally for this the Traits from strum are used. 
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
