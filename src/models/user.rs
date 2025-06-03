use mysql::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub age: Option<i32>,
}

impl User {
    pub fn new(username: String, email: String, age: Option<i32>) -> Self {
        User {
            id: None,
            username,
            email,
            age,
        }
    }
}
