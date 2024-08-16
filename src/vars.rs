use std::{
    cell::RefCell,
    sync::{Mutex, OnceLock},
};

use crate::widgets::AppWidgets;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub fullname: String,
}

impl User {
    pub fn new(username: String, fullname: String) -> Self {
        User { username, fullname }
    }
}

thread_local! {
    pub static APP_WIDGETS: RefCell<Option<AppWidgets>> = RefCell::new(None);
}

pub static USERS: OnceLock<Vec<User>> = OnceLock::new();

pub static SESSIONS: OnceLock<Vec<(String, String)>> = OnceLock::new();

pub static SELECTED_USER: OnceLock<Mutex<usize>> = OnceLock::new();
