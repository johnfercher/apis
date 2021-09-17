use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::errors::{ErrorType, AppError};
use crate::models::Order;
use crate::errors::ErrorType::NotFound;

type PooledPg = PooledConnection<ConnectionManager<SqliteConnection>>;

pub struct OrderRepository {
    connection: PooledPg,
}

impl OrderRepository {
    pub fn new(connection: PooledPg) -> OrderRepository {
        OrderRepository {connection}
    }

    pub fn create_order(&self, dto: Order) -> Result<usize, AppError> {
        use super::schema::orders;

        diesel::insert_into(orders::table)
            .values(&dto)
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while creating book")
            })
    }

    pub fn get_order_by_id(&self, order_id: String) -> Result<Order, AppError> {
        use super::schema::orders::dsl::*;

        let result: Vec<Order> = orders
            .filter(id.eq(order_id))
            .load(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while listing orders")
            })?;

        match result.first() {
            None => Err(AppError::new("while listing orders", NotFound)),
            Some(v) => Ok(v.clone())
        }
    }

    pub fn list_orders(&self) -> Result<Vec<Order>, AppError> {
        use super::schema::orders::dsl::*;

        orders
            .load(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while listing orders")
            })
    }

    pub fn update_orders(&self, dto: Order) -> Result<usize, AppError> {
        use super::schema::orders::dsl::*;

        let updated = diesel::update(&dto)
            .filter(id.eq(dto.id.clone()))
            .set(label_code.eq(dto.label_code.clone()))
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while updating book status")
            })?;

        if updated == 0 {
            return Err(AppError::new("Book not found", ErrorType::NotFound))
        }
        return Ok(updated)
    }

    pub fn delete_order(&self, order_id: String) -> Result<usize, AppError> {
        use super::schema::orders::dsl::*;

        let deleted = diesel::delete(orders.filter(id.eq(order_id)))
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while deleting book")
            })?;

        if deleted == 0 {
            return Err(AppError::new("Book not found", ErrorType::NotFound))
        }

        return Ok(deleted)
    }
}