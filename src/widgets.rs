use gtk::{
    prelude::*, ApplicationWindow, Builder, Button, ComboBoxText, Entry, Label, ListBox,
    ListBoxRow, ModelButton, Stack,
};

pub struct AppWidgets {
    pub window: ApplicationWindow,
    pub lbl_hostname: Label,
    pub combo_session_types: ComboBoxText,
    pub btn_power_poweroff: ModelButton,
    pub btn_power_restart: ModelButton,
    pub lbl_time: Label,
    pub lbl_date: Label,
    pub stack: Stack,
    pub btn_not_listed: Button,
    pub btn_back_to_user_list: Button,
    pub list_box_users: ListBox,
    pub btn_switch_user: Button,
    pub lbl_selected_user_fullname: Label,
    pub lbl_selected_user_username: Label,
    pub btn_selected_user_login: Button,
    pub entry_selected_user_passwd: Entry,
    pub lbl_selected_user_error_message: Label,
    pub entry_hidden_user_username: Entry,
    pub entry_hidden_user_passwd: Entry,
    pub btn_hidden_user_login: Button,
    pub lbl_hidden_user_error_message: Label,
}

impl AppWidgets {
    pub fn new(builder: &gtk::Builder) -> Self {
        Self {
            window: builder
                .object("main_window")
                .expect("Failed to get main_window"),
            lbl_hostname: builder
                .object("lbl_hostname")
                .expect("Failed to get lbl_hostname"),
            combo_session_types: builder
                .object("combo_session_types")
                .expect("Failed to get combo_session_types"),
            btn_power_poweroff: builder
                .object("btn_power_poweroff")
                .expect("Failed to get btn_power_poweroff"),
            btn_power_restart: builder
                .object("btn_power_restart")
                .expect("Failed to get btn_power_restart"),
            lbl_time: builder.object("lbl_time").expect("Failed to get lbl_time"),
            lbl_date: builder.object("lbl_date").expect("Failed to get lbl_date"),
            stack: builder
                .object("stack_main")
                .expect("Failed to get stack_main"),
            btn_not_listed: builder
                .object("btn_not_listed")
                .expect("Failed to get btn_not_listed"),
            btn_back_to_user_list: builder
                .object("btn_back_to_user_list")
                .expect("Failed to get btn_back_to_user_list"),
            list_box_users: builder
                .object("list_box_users")
                .expect("Failed to get list_box_users"),
            btn_switch_user: builder
                .object("btn_switch_user")
                .expect("Failed to get btn_switch_user"),
            lbl_selected_user_fullname: builder
                .object("lbl_selected_user_fullname")
                .expect("Failed to get lbl_selected_user_fullname"),
            lbl_selected_user_username: builder
                .object("lbl_selected_user_username")
                .expect("Failed to get lbl_selected_user_username"),
            btn_selected_user_login: builder
                .object("btn_selected_user_login")
                .expect("Failed to get btn_selected_user_login"),
            entry_selected_user_passwd: builder
                .object("entry_selected_user_passwd")
                .expect("Failed to get entry_selected_user_passwd"),
            lbl_selected_user_error_message: builder
                .object("lbl_selected_user_error_message")
                .expect("Failed to get lbl_error_message"),
            entry_hidden_user_username: builder
                .object("entry_hidden_user_username")
                .expect("Failed to get entry_hidden_user_username"),
            entry_hidden_user_passwd: builder
                .object("entry_hidden_user_passwd")
                .expect("Failed to get entry_hidden_user_passwd"),
            btn_hidden_user_login: builder
                .object("btn_hidden_user_login")
                .expect("Failed to get btn_hidden_user_login"),
            lbl_hidden_user_error_message: builder
                .object("lbl_hidden_user_error_message")
                .expect("Failed to get lbl_hidden_user_error_message"),
        }
    }
}

pub struct UserListItem {
    pub box_row: ListBoxRow,
}

impl UserListItem {
    pub fn new(username: &str, full_name: &str) -> Self {
        let user_list_item_ui = include_str!("../resources/ui/user_list_item.ui");
        let builder = Builder::from_string(user_list_item_ui);
        let box_row = ListBoxRow::new();
        let user_list_item: gtk::Box = builder
            .object("user_list_item")
            .expect("Failed to get user_list_item");
        let lbl_user_fullname: Label = builder
            .object("lbl_user_fullname")
            .expect("Failed to get lbl_user_fullname");
        let lbl_user_username: Label = builder
            .object("lbl_user_username")
            .expect("Failed to get lbl_user_username");

        if full_name.is_empty() {
            lbl_user_fullname.set_text(username);
            lbl_user_username.hide();
        } else {
            lbl_user_fullname.set_text(full_name);
            lbl_user_username.set_text(username);
            lbl_user_username.show();
        }

        box_row.add(&user_list_item);

        Self { box_row }
    }

    pub fn get_widget(&self) -> &ListBoxRow {
        &self.box_row
    }
}
