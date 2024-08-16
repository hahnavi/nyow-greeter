use std::time::Duration;

use chrono::Local;
use gtk::{
    gdk::Screen,
    glib::{timeout_add_local, ControlFlow},
    prelude::*,
    Application, Builder, CssProvider, StyleContext,
};
use log::error;

use crate::{
    callbacks::{select_user, setup_callbacks},
    helpers::get_widgets,
    util,
    vars::{APP_WIDGETS, SESSIONS, USERS},
    widgets::{AppWidgets, UserListItem},
};

pub fn setup_ui(app: &Application) -> Result<(), Box<dyn std::error::Error>> {
    let window_ui = include_str!("../resources/ui/window.ui");
    let builder = Builder::from_string(window_ui);

    APP_WIDGETS.with(|app_widgets| {
        *app_widgets.borrow_mut() = Some(AppWidgets::new(&builder));
    });

    let window = get_widgets(|w| w.window.clone());
    window.set_application(Some(app));
    window.set_decorated(false);

    setup_hostname();
    setup_session_types();
    update_date_time();
    load_css()?;
    setup_callbacks();
    start_date_time_update();
    setup_stack();
    setup_user_list();

    window.show_all();

    Ok(())
}

fn setup_hostname() {
    let lbl_hostname = get_widgets(|w| w.lbl_hostname.clone());
    lbl_hostname.set_text(&util::get_hostname());
}

fn setup_session_types() {
    let combo_session_types = get_widgets(|w| w.combo_session_types.clone());
    if let Some(sessions) = SESSIONS.get() {
        for session in sessions.iter() {
            combo_session_types.append_text(&session.0);
        }
        combo_session_types.set_active(Some(0));
    } else {
        error!("SESSIONS is not initialized.");
    }
}

fn update_date_time() {
    get_widgets(|w| {
        let now = Local::now();
        w.lbl_time.set_text(&now.format("%H:%M").to_string());
        w.lbl_date.set_text(&now.format("%A, %d %B %Y").to_string());
    });
}

fn load_css() -> Result<(), Box<dyn std::error::Error>> {
    let css = include_str!("../resources/styles/style.css");
    let provider = CssProvider::new();
    provider.load_from_data(css.as_bytes())?;
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    Ok(())
}

fn start_date_time_update() {
    timeout_add_local(Duration::from_secs(1), move || {
        update_date_time();
        ControlFlow::Continue
    });
}

fn setup_stack() {
    let users = USERS.get().unwrap();
    if users.is_empty() {
        get_widgets(|w| w.stack.set_visible_child_name("page2"));
        reset_hidden_user_widgets();
    } else if users.len() == 1 {
        select_user(0);
        reset_selected_user_widgets();
    } else {
        get_widgets(|w| w.stack.set_visible_child_name("page0"));
        reset_user_list_widgets();
    }

    get_widgets(|w| w.stack.clone()).connect_notify(Some("visible-child"), move |_, _| {
        if let Some(visible_child) = get_widgets(|w| w.stack.clone()).visible_child_name() {
            match visible_child.as_str() {
                "page0" => {
                    reset_user_list_widgets();
                }
                "page1" => {
                    reset_selected_user_widgets();
                }
                "page2" => {
                    reset_hidden_user_widgets();
                }
                _ => {}
            }
        }
    });
}

fn setup_user_list() {
    let list_box_users = get_widgets(|w| w.list_box_users.clone());
    if let Some(users) = USERS.get() {
        for user in users.iter() {
            let item = UserListItem::new(&user.username, &user.fullname);
            list_box_users.add(item.get_widget());
        }
        list_box_users.connect_row_activated(move |_, row| {
            let row_index = row.index() as usize;
            select_user(row_index);
        });
        if !users.is_empty() {
            if let Some(first_row) = list_box_users.row_at_index(0) {
                first_row.grab_focus();
                list_box_users.select_row(Some(&first_row));
            }
        }
    } else {
        error!("USERS is not initialized.");
    }
}

pub fn reset_user_list_widgets() {
    get_widgets(|w| {
        w.combo_session_types.set_visible(false);
    });
}

pub fn reset_selected_user_widgets() {
    get_widgets(|w| {
        w.combo_session_types.set_visible(true);
        w.entry_selected_user_passwd.set_text("");
        w.entry_selected_user_passwd.grab_focus();
        w.lbl_selected_user_error_message.set_visible(false);
    });
}

pub fn reset_hidden_user_widgets() {
    get_widgets(|w| {
        w.combo_session_types.set_visible(true);
        w.entry_hidden_user_username.set_text("");
        w.entry_hidden_user_passwd.set_text("");
        w.entry_hidden_user_username.grab_focus();
        w.lbl_hidden_user_error_message.set_visible(false);
    });
}
