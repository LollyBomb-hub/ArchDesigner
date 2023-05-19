use crate::models::account::{Account};
use crate::schema::accounts;

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = accounts)]
struct NewAccount<'a> {
    name: &'a String,
    password: &'a String
}

pub fn create(conn: &mut PgConnection, name: &String, password: &String) -> Account {
    let new_account = NewAccount {
        name: name,
        password: password
    };

    diesel::insert_into(accounts::table)
        .values(new_account)
        .returning(
            (
                accounts::account_id,
                accounts::name,
                accounts::created_at,
                accounts::stars,
                accounts::about
            )
        )
        .get_result::<Account>(conn)
        .expect("Error creating account")
}

pub fn list(conn: &mut PgConnection, limit: Option<i64>) -> Vec<Account> {
    accounts::dsl::accounts.select(
        (
            accounts::account_id,
            accounts::name,
            accounts::created_at,
            accounts::stars,
            accounts::about
        )
    ).limit(limit.unwrap_or(10i64))
        .load::<Account>(conn)
        .expect("Error loading!")
}

pub fn get_account_by_auth_data(conn: &mut PgConnection, username: &String, password: &String) -> Vec<Account> {
    accounts::dsl::accounts.select(
        (
            accounts::account_id,
            accounts::name,
            accounts::created_at,
            accounts::stars,
            accounts::about
        )
    ).filter(accounts::dsl::name.eq(username).and(accounts::dsl::password.eq(password))).load::<Account>(conn).expect("Error searching!")
}

pub fn get_account(conn: &mut PgConnection, account_id: i32) -> Account {
    accounts::table.select(
        (
            accounts::account_id,
            accounts::name,
            accounts::created_at,
            accounts::stars,
            accounts::about
        )
    ).find(account_id).first::<Account>(conn).expect("Could not get account by id")

}