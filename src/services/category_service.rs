use actix_web::web;
use diesel::{Connection, SqliteConnection};
use r2d2::PooledConnection;

use crate::{
    core::{
        commands::{
            create_categories::CreateCategoriesCommand, update_categories::UpdateCategoriesCommand,
        },
        entities::categories::Categories,
    },
    infrastructure::repositories::category_repository::CategoryRepository,
    utils::errors::ApiError,
};

pub struct CategoryService {
    pub category_repo: CategoryRepository,
    pub conn: PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>,
}

impl CategoryService {
    #[allow(dead_code)]
    pub fn create_category(
        &mut self,
        cmd: CreateCategoriesCommand,
    ) -> Result<Categories, ApiError> {
        self.conn
            .transaction(|txn_conn| {
                if let Some(_) = self.category_repo.find_category_name(txn_conn, &cmd.name)? {
                    return Err(ApiError {
                        message: "Category already exists".to_string(),
                        error: None,
                    });
                }

                let category = Categories::new(&cmd.name);
                self.category_repo.create_category(txn_conn, &category)?;
                Ok(category)
            })
            .map_err(|e| e.into())
    }

    #[allow(dead_code)]
    pub fn update_category(
        &mut self,
        cmd: UpdateCategoriesCommand,
        path: web::Path<String>,
    ) -> Result<Categories, ApiError> {
        self.conn
            .transaction(|txn_conn| {
                if let Some(_) = self.category_repo.find_category_name(txn_conn, &cmd.name)? {
                    return Err(ApiError {
                        message: "Category already exists".to_string(),
                        error: None,
                    });
                }

                let category = Categories::new(&cmd.name);
                self.category_repo
                    .update_category(txn_conn, &path, &cmd.name)?;
                Ok(category)
            })
            .map_err(|e| e.into())
    }
}
