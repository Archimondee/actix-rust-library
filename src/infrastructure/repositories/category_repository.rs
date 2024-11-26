use crate::core::entities::categories::Categories;
use crate::utils::log_query;
use diesel::query_dsl::methods::FilterDsl;
use diesel::result::Error;
use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, SqliteConnection};

pub struct CategoryRepository;

impl CategoryRepository {
    #[allow(dead_code)]
    pub fn create_category(
        &mut self,
        conn: &mut SqliteConnection,
        category: &Categories,
    ) -> Result<usize, Error> {
        use crate::infrastructure::schema::schema::categories::dsl::*;
        let query = diesel::insert_into(categories).values(category);
        log_query(query, || query.execute(conn))
    }

    #[allow(dead_code)]
    pub fn find_category_id(
        &mut self,
        conn: &mut SqliteConnection,
        id_category: &str,
    ) -> Result<Option<Categories>, Error> {
        use crate::infrastructure::schema::schema::categories::dsl::*;
        let query = categories.filter(id.eq(id_category));
        log_query(query, || query.first(conn).optional())
    }

    #[allow(dead_code)]
    pub fn find_category_name(
        &mut self,
        conn: &mut SqliteConnection,
        name_category: &str,
    ) -> Result<Option<Categories>, Error> {
        use crate::infrastructure::schema::schema::categories::dsl::*;
        let query = categories.filter(name.eq(name_category));
        log_query(query, || query.first(conn).optional())
    }

    #[allow(dead_code)]
    pub fn update_category(
        &mut self,
        conn: &mut SqliteConnection,
        id_category: &str,
        name_category: &str,
    ) -> Result<Option<Categories>, Error> {
        use crate::infrastructure::schema::schema::categories::dsl::*;
        let query =
            diesel::update(categories.filter(id.eq(id_category))).set(name.eq(name_category));
        let updated_count = log_query(query.clone(), || query.execute(conn));

        if updated_count.is_ok() {
            let query = categories.filter(id.eq(id_category));
            log_query(query, || query.first(conn).optional())
        } else {
            Ok(None)
        }
    }

    #[allow(dead_code)]
    pub fn delete_category(
        &mut self,
        conn: &mut SqliteConnection,
        id_category: &str,
    ) -> Result<usize, Error> {
        use crate::infrastructure::schema::schema::categories::dsl::*;
        let query = diesel::delete(categories.filter(id.eq(id_category)));
        log_query(query.clone(), || query.execute(conn))
    }
}
