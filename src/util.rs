use gtk::glib::ffi::g_get_host_name;
use std::{
    collections::HashSet,
    ffi::CStr,
    fs::{self},
    io::Result,
    path::Path,
    process::Command,
};

use crate::vars::User;

pub fn get_hostname() -> String {
    unsafe { CStr::from_ptr(g_get_host_name()) }
        .to_string_lossy()
        .into_owned()
}

pub fn poweroff() {
    Command::new("poweroff")
        .output()
        .expect("Failed to execute command");
}

pub fn restart() {
    Command::new("reboot")
        .output()
        .expect("Failed to execute command");
}

pub fn get_session_types() -> Vec<(String, String)> {
    let mut sessions = Vec::new();

    let session_dirs = [
        "/usr/share/xsessions/",
        "/usr/share/wayland-sessions/",
        "/usr/local/share/xsessions/",
        "/usr/local/share/wayland-sessions/",
    ];

    for dir in session_dirs.iter() {
        let session_dir = Path::new(dir);

        if session_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(session_dir) {
                for entry in entries.filter_map(Result::ok) {
                    if let Some(_file_name) = entry.path().file_stem() {
                        let file_path = entry.path();
                        if let Ok(content) = fs::read_to_string(file_path) {
                            let name_line = content.lines().find(|line| line.starts_with("Name="));
                            let exec_line = content.lines().find(|line| line.starts_with("Exec="));

                            if let (Some(name_line), Some(exec_line)) = (name_line, exec_line) {
                                let name =
                                    name_line.strip_prefix("Name=").unwrap_or("").to_string();
                                let exec =
                                    exec_line.strip_prefix("Exec=").unwrap_or("").to_string();
                                sessions.push((name, exec));
                            }
                        }
                    }
                }
            }
        }
    }

    sessions
}

pub fn get_user_list() -> Vec<User> {
    let minimum_uid = 1000;
    let hidden_users: HashSet<&str> = ["nobody", "nobody4", "noaccess"].iter().cloned().collect();
    let hidden_shells: HashSet<&str> = ["/bin/false", "/usr/sbin/nologin", "/sbin/nologin"]
        .iter()
        .cloned()
        .collect();

    let mut users = Vec::new();
    let path = Path::new("/etc/passwd");

    if let Ok(contents) = fs::read_to_string(path) {
        for line in contents.lines() {
            let fields: Vec<&str> = line.split(':').collect();
            if fields.len() > 6 {
                let username = fields[0].to_string();
                let uid: u32 = fields[2].parse().unwrap_or(0);
                let shell = fields[6];
                let full_name = fields[4].to_string();

                if uid >= minimum_uid
                    && !hidden_users.contains(username.as_str())
                    && !hidden_shells.contains(shell)
                {
                    users.push(User::new(username, full_name));
                }
            }
        }
    }

    users
}
