use gtk::{prelude::*, Label};
use log::error;
use std::process::exit;

use crate::{
    greetd,
    helpers::get_widgets,
    setup_ui::reset_selected_user_widgets,
    util,
    vars::{SELECTED_USER, SESSIONS, USERS},
};

pub fn setup_callbacks() {
    setup_power_buttons();
    setup_login_button();
    setup_navigation_buttons();
}

fn setup_power_buttons() {
    get_widgets(|w| {
        w.btn_power_poweroff.clone().connect_clicked(|_| {
            util::poweroff();
        });
        w.btn_power_restart.clone().connect_clicked(|_| {
            util::restart();
        });
    });
}

fn setup_navigation_buttons() {
    get_widgets(|w| {
        w.btn_not_listed.clone().connect_clicked(move |_| {
            get_widgets(|w| w.stack.set_visible_child_name("page2"));
        });
        w.btn_switch_user.clone().connect_clicked(move |_| {
            if let Some(users) = USERS.get() {
                if users.len() == 1 {
                    get_widgets(|w| w.stack.set_visible_child_name("page2"));
                } else {
                    get_widgets(|w| w.stack.set_visible_child_name("page0"));
                }
            } else {
                error!("USERS is not initialized.");
            }
        });
        w.btn_back_to_user_list.clone().connect_clicked(move |_| {
            get_widgets(|w| w.stack.set_visible_child_name("page0"));
        });
    });
}

fn setup_login_button() {
    get_widgets(|w| {
        w.btn_selected_user_login.clone().connect_clicked(move |_| {
            selected_user_login();
        });
        w.entry_selected_user_passwd
            .clone()
            .connect_activate(move |_| selected_user_login());
        w.btn_hidden_user_login.clone().connect_clicked(move |_| {
            hidden_user_login();
        });
        w.entry_hidden_user_username
            .clone()
            .connect_activate(move |_| {
                get_widgets(|w| w.entry_hidden_user_passwd.grab_focus());
            });
        w.entry_hidden_user_passwd
            .clone()
            .connect_activate(move |_| hidden_user_login());
    });
}

fn selected_user_login() {
    let selected_user = SELECTED_USER.get().unwrap().lock().unwrap();
    let username = USERS.get().unwrap()[*selected_user].username.clone();
    let password = get_widgets(|w| w.entry_selected_user_passwd.text().to_string());
    let session_index = get_widgets(|w| w.combo_session_types.active().unwrap() as usize);
    match login(username, password, session_index) {
        Ok(_) => {
            exit(0);
        }
        Err(error) => {
            show_error_message(
                get_widgets(|w| w.lbl_selected_user_error_message.clone()),
                &error,
            );
            return;
        }
    }
}

fn hidden_user_login() {
    let username = get_widgets(|w| w.entry_hidden_user_username.text().to_string());
    let password = get_widgets(|w| w.entry_hidden_user_passwd.text().to_string());
    let session_index = get_widgets(|w| w.combo_session_types.active().unwrap() as usize);
    match login(username, password, session_index) {
        Ok(_) => {
            exit(0);
        }
        Err(error) => {
            show_error_message(
                get_widgets(|w| w.lbl_hidden_user_error_message.clone()),
                &error,
            );
            return;
        }
    }
}

fn show_error_message(target: Label, message: &str) {
    target.set_visible(true);
    target.set_text(message);
}

pub fn select_user(index: usize) {
    get_widgets(|w| {
        w.stack.set_visible_child_name("page1");
        reset_selected_user_widgets();
        let users = USERS.get().unwrap();
        if users[index].fullname.is_empty() {
            w.lbl_selected_user_fullname
                .set_text(&users[index].username);
            w.lbl_selected_user_username.hide();
        } else {
            w.lbl_selected_user_fullname
                .set_text(&users[index].fullname);
            w.lbl_selected_user_username
                .set_text(&users[index].username);
            w.lbl_selected_user_username.show();
        }
        w.entry_selected_user_passwd.grab_focus();
        let mut selected_user = SELECTED_USER.get().unwrap().lock().unwrap();
        *selected_user = index.into();
    });
}

pub fn login(username: String, password: String, session_index: usize) -> Result<(), String> {
    let sessions = SESSIONS.get().ok_or("Failed to get sessions")?;

    let mut greetd = greetd::GreetD::new().map_err(|error| {
        error!("Failed to connect to greetd: {:?}", error);
        error.to_string()
    })?;

    greetd
        .login(username, password, vec![sessions[session_index].1.clone()])
        .map_err(|error| {
            greetd.cancel();
            error!("Failed to login: {:?}", error);
            error.to_string()
        })?;

    Ok(())
}
