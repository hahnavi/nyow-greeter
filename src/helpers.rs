use crate::{vars::APP_WIDGETS, widgets::AppWidgets};

pub fn get_widgets<F, T>(f: F) -> T
where
    F: Fn(&AppWidgets) -> T,
{
    APP_WIDGETS.with(|app_widgets| {
        let app_widgets = app_widgets.borrow();
        f(app_widgets.as_ref().expect("AppWidgets not initialized"))
    })
}
