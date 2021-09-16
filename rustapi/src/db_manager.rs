use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::errors::{ErrorType, AppError};
use crate::models::Order;

type PooledPg = PooledConnection<ConnectionManager<SqliteConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager {connection}
    }

    pub fn create_book(&self, dto: Order) -> Result<usize, AppError> {
        use super::schema::orders;

        diesel::insert_into(orders::table) // insert into books table
            .values(&dto) // use values from CreateBookDTO
            .execute(&self.connection) // execute query
            .map_err(|err| {
                AppError::from_diesel_err(err, "while creating book")
            }) // if error occurred map it to AppError
    }

    pub fn list_orders(&self) -> Result<Vec<Order>, AppError> {
        use super::schema::orders::dsl::*;

        orders
            .load(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while listing books")
            })
    }
}