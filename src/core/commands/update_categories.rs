use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone)]
pub struct UpdateCategoriesCommand {
    #[validate(length(min = 3, message = "Name must be at least 3 character long"))]
    pub name: String,
}
