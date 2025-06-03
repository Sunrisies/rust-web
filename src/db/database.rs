use mysql::*;
use mysql::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::User;
use super::Result;

#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

pub struct PaginatedUsers {
    pub users: Vec<User>,
    pub total: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub limit: i64,
}

impl Database {
    pub fn new() -> Result<Self> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let opts = Opts::from_url(&database_url)?;
        let pool = Pool::new(opts)?;
        Ok(Database { pool })
    }

    // 创建用户表
    pub fn create_table(&self) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            r"CREATE TABLE IF NOT EXISTS users (
                id INT PRIMARY KEY AUTO_INCREMENT,
                username VARCHAR(255) NOT NULL UNIQUE,
                email VARCHAR(255) NOT NULL,
                age INT
            )",
            (),
        )?;
        Ok(())
    }

    // 检查用户名是否存在
    pub fn username_exists(&self, username: &str) -> Result<bool> {
        let mut conn = self.pool.get_conn()?;
        let count: Option<i32> = conn.exec_first(
            "SELECT COUNT(*) FROM users WHERE username = :username",
            params! {
                "username" => username,
            },
        )?;
        
        Ok(count.unwrap_or(0) > 0)
    }

    // 获取分页用户列表
    pub fn get_users_paginated(&self, page: i64, limit: i64) -> Result<PaginatedUsers> {
        let mut conn = self.pool.get_conn()?;

        // 获取总记录数
        let total: i64 = conn
            .query_first("SELECT COUNT(*) FROM users")?
            .unwrap_or(0);

        // 计算总页数
        let total_pages = (total + limit - 1) / limit;
        
        // 确保页码在有效范围内
        let current_page = if page <= 0 {
            1
        } else if page > total_pages {
            total_pages.max(1) // 如果没有数据，至少返回第1页
        } else {
            page
        };

        // 计算偏移量
        let offset = (current_page - 1) * limit;

        // 获取当前页的用户数据
        let users = conn.exec_map(
            "SELECT id, username, email, age FROM users LIMIT :offset, :limit",
            params! {
                "offset" => offset,
                "limit" => limit,
            },
            |(id, username, email, age)| User {
                id: Some(id),
                username,
                email,
                age,
            },
        )?;

        Ok(PaginatedUsers {
            users,
            total,
            total_pages,
            current_page,
            limit,
        })
    }

    // 创建新用户
    pub fn create_user(&self, user: &User) -> Result<u64> {
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

    // 通过ID获取用户
    pub fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>> {
        let mut conn = self.pool.get_conn()?;
        let user = conn.exec_first(
            "SELECT id, username, email, age FROM users WHERE id = :id",
            params! { "id" => user_id },
        )?;
        Ok(user)
    }

    // 更新用户信息
    pub fn update_user(&self, user: &User) -> Result<()> {
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
    pub fn delete_user(&self, user_id: i32) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "DELETE FROM users WHERE id = :id",
            params! { "id" => user_id },
        )?;
        Ok(())
    }
}