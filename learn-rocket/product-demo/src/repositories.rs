use rocket::futures::TryStreamExt;
use sqlx::{
    error::Error,
    pool::PoolConnection,
    sqlite::{Sqlite, SqliteQueryResult},
};

use crate::models::{NewProduct, Product};

type Conn = PoolConnection<Sqlite>;
type QueryResult<T> = Result<T, Error>;

pub struct ProductsRepo;

impl ProductsRepo {
    pub async fn find(c: &mut Conn, id: i64) -> QueryResult<Product> {
        let sql = "SELECT * FROM products where id = ?";

        sqlx::query_as::<_, Product>(sql)
            .bind(id)
            .fetch_one(c)
            .await
    }

    pub async fn find_all(c: &mut Conn) -> QueryResult<Vec<Product>> {
        let sql = "SELECT id, title, description FROM products";

        sqlx::query_as::<_, Product>(sql)
            .fetch(c)
            .try_collect::<Vec<_>>()
            .await
    }

    pub async fn create(c: &mut Conn, product: NewProduct) -> QueryResult<Product> {
        let sql = "INSERT INTO products (title, description) VALUES (?, ?)";

        let result = sqlx::query(sql)
            .bind(product.title)
            .bind(product.description.map_or(String::new(), |desc| desc))
            .execute(&mut *c)
            .await?;

        let last_id: i64 = Self::last_id(&result)?;
        Self::find(c, last_id).await
    }

    pub async fn create_all(c: &mut Conn, products: Vec<NewProduct>) -> QueryResult<Vec<Product>> {
        let mut result = vec![];

        for ele in products {
            let product = Self::create(c, ele).await?;
            result.push(product);
        }

        Ok(result)
    }

    pub fn update() -> QueryResult<Product> {
        todo!()
    }

    pub async fn delete(c: &mut Conn, id: i64) -> QueryResult<Product> {
        let product = Self::find(c, id).await?;

        let sql = format!("DELETE FROM products where id = {id}");
        sqlx::query(&sql).execute(c).await?;

        Ok(product)
    }
}

impl ProductsRepo {
    fn last_id(result: &SqliteQueryResult) -> Result<i64, Error> {
        if result.rows_affected() > 0 {
            let id = result.last_insert_rowid();
            Ok(id)
        } else {
            Err(Error::ColumnNotFound("Last Row".to_string()))
        }
    }
}
