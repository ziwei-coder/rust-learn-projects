use rocket::futures::TryStreamExt;
use sqlx::pool::PoolConnection;
use sqlx::Sqlite;

use crate::models::Product;

const TABLE: &str = "products";

type QueryResult<T> = Result<T, sqlx::error::Error>;

pub struct ProductsRepo;

impl ProductsRepo {
    pub async fn find_all(c: &mut PoolConnection<Sqlite>) -> QueryResult<Vec<Product>> {
        let sql = format!("SELECT id, title, description FROM {TABLE}");
        sqlx::query_as::<_, Product>(&sql)
            .fetch(c)
            .try_collect::<Vec<_>>()
            .await
    }

    pub async fn find(c: &mut PoolConnection<Sqlite>, id: i32) -> QueryResult<Product> {
        let sql = format!("SELECT * FROM {TABLE} where id = {id}");
        sqlx::query_as::<_, Product>(&sql).fetch_one(c).await
    }

    pub async fn delete(c: &mut PoolConnection<Sqlite>, id: i32) -> QueryResult<Product> {
        let product = Self::find(c, id).await?;

        let sql = format!("DELETE FROM {TABLE} where id = {id}");
        sqlx::query(&sql).execute(c).await?;

        Ok(product)
    }
}
