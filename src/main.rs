use mysql::*;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, FromRow)]
struct User {
    id: Option<i32>,
    username: String,
    email: String,
    age: Option<i32>,
}

struct Database {
    pool: Pool,
}

impl Database {
    fn new() -> Result<Self> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let opts = Opts::from_url(&database_url).expect("Invalid DATABASE_URL");
        let pool = Pool::new(opts)?;
        Ok(Database { pool })
    }

    // 创建用户表
    fn create_table(&self) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            r"CREATE TABLE IF NOT EXISTS users (
                id INT PRIMARY KEY AUTO_INCREMENT,
                username VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                age INT
            )",
            (),
        )?;
        Ok(())
    }

    // 创建新用户
    fn create_user(&self, user: &User) -> Result<u64> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "INSERT INTO users (username, email, age) VALUES (:username, :email, :age)",
            params! {
                "username" => &user.username,
                "email" => &user.email,
                "age" => user.age,
            },
        )?;
        Ok(conn.last_insert_id())
    }

    // 获取所有用户
    fn get_all_users(&self) -> Result<Vec<User>> {
        let mut conn = self.pool.get_conn()?;
        let users = conn.query_map(
            "SELECT id, username, email, age FROM users",
            |(id, username, email, age)| User {
                id: Some(id),
                username,
                email,
                age,
            },
        )?;
        Ok(users)
    }

    // 通过ID获取用户
    fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>> {
        let mut conn = self.pool.get_conn()?;
        let user = conn.exec_first(
            "SELECT id, username, email, age FROM users WHERE id = :id",
            params! { "id" => user_id },
        )?;
        Ok(user)
    }

    // 更新用户信息
    fn update_user(&self, user: &User) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "UPDATE users SET username = :username, email = :email, age = :age WHERE id = :id",
            params! {
                "id" => user.id,
                "username" => &user.username,
                "email" => &user.email,
                "age" => user.age,
            },
        )?;
        Ok(())
    }

    // 删除用户
    fn delete_user(&self, user_id: i32) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "DELETE FROM users WHERE id = :id",
            params! { "id" => user_id },
        )?;
        Ok(())
    }
}

fn main() -> Result<()> {
    // 创建数据库连接
    let db = Database::new()?;
    
    // 创建用户表
    db.create_table()?;

    // 创建新用户
    let new_user = User {
        id: None,
        username: String::from("john_doe"),
        email: String::from("john@example.com"),
        age: Some(25),
    };
    
    println!("Creating new user...");
    let user_id = db.create_user(&new_user)?;
    println!("Created user with ID: {}", user_id);

    // 获取所有用户
    println!("\nAll users:");
    let users = db.get_all_users()?;
    for user in users {
        println!("{:?}", user);
    }

    // 获取特定用户
    println!("\nGetting user by ID {}:", user_id);
    if let Some(user) = db.get_user_by_id(user_id as i32)? {
        println!("Found user: {:?}", user);

        // 更新用户
        let mut updated_user = user;
        updated_user.age = Some(26);
        println!("\nUpdating user...");
        db.update_user(&updated_user)?;
        println!("User updated successfully!");

        // 验证更新
        println!("\nVerifying update:");
        if let Some(verified_user) = db.get_user_by_id(user_id as i32)? {
            println!("Updated user: {:?}", verified_user);
        }

        // 删除用户
        // println!("\nDeleting user...");
        // db.delete_user(user_id as i32)?;
        // println!("User deleted successfully!");
    }

    Ok(())
}