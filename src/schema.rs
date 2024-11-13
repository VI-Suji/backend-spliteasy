use serde::{Deserialize, Serialize}; 
use uuid::Uuid;
use sqlx::types::BigDecimal;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize)]
struct Users {
	user_id: i32,
	user_name: String,
	user_email: String,
	created_at: String,
	updated_at: String
}


#[derive(Serialize)]
struct Groups {
    group_id: i32,
    group_name: String,
    created_at: String,
	updated_at: String
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct Splits {
    pub split_id: Uuid,
    pub split_user: Option<i32>,                 
    pub split_group: Option<i32>,        
    pub split_type: Option<i32>, 
    #[serde_as(as = "DisplayFromStr")]       
    pub expense: BigDecimal,                
    pub description: Option<String>,     
}

#[derive(Serialize)]
pub struct Type {
	pub user_id: Option<i32>,
	pub type_name: String,
	pub type_id: i32
}
