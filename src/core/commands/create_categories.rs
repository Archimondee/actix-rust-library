use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone)]
pub struct CreateCategoriesCommand {
    #[validate(length(min = 3, message = "Name must be at least 3 character long"))]
    pub name: String,
}
