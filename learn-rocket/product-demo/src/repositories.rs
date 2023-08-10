use diesel::prelude::*;

use crate::models::{NewProduct, Product};
use crate::schema::products;

pub struct ProductRepo;

impl ProductRepo {
    pub fn find_all(c: &mut SqliteConnection) -> QueryResult<Vec<Product>> {
        products::table.limit(100).load::<Product>(c)
    }

    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Product> {
        products::table.find(id).get_result::<Product>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_product: NewProduct) -> QueryResult<Product> {
        diesel::insert_into(products::table)
            .values(new_product)
            .execute(c)?;

        let last_id = Self::last_id(c)?;
        Self::find(c, last_id)
    }

    pub fn update(c: &mut SqliteConnection, product: Product) -> QueryResult<Product> {
        diesel::update(products::table.find(product.id))
            .set((
                products::title.eq(product.title.to_owned()),
                products::description.eq(product.description.to_owned()),
            ))
            .execute(c)?;

        Self::find(c, product.id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(products::table.find(id)).execute(c)
    }

    fn last_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        products::table
            .select(products::id)
            .order(products::id.desc())
            .first(c)
    }
}
