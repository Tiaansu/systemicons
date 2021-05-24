use std::ffi::{CStr, CString, c_void};
use gio_sys::GThemedIcon;
use glib::{gobject_sys::g_object_unref, object::GObject};
use glib_sys::g_free;
use gtk_sys::{GtkIconTheme, gtk_icon_info_get_filename, gtk_icon_theme_choose_icon, gtk_icon_theme_get_default};

static mut DEFAULT_THEME: Option<*mut GtkIconTheme> = None;

pub fn get_icon(ext: &str) -> String {
    let result: String;
    unsafe {
        let filename = CString::new(ext).unwrap();
        let null: u8 = 0;
        let p_null = &null as *const u8;
        let nullsize: usize = 0;
        let mut res = 0;
        let p_res = &mut res as *mut i32;
        let p_res = gio_sys::g_content_type_guess(filename.as_ptr(), p_null, nullsize, p_res);
        let icon = gio_sys::g_content_type_get_icon(p_res);
        g_free(p_res as *mut c_void);
        if DEFAULT_THEME.is_none() {
            DEFAULT_THEME = Some(gtk_icon_theme_get_default());
        }
        let icon_names = gio_sys::g_themed_icon_get_names(icon as *mut GThemedIcon) as *mut *const i8;
        let icon_info = gtk_icon_theme_choose_icon(DEFAULT_THEME.unwrap(), icon_names, 16, 2);
        let filename = gtk_icon_info_get_filename(icon_info);
        let res_str = CStr::from_ptr(filename);
        result = match res_str.to_str() {
            Ok(str) => str.to_string(),
            Err(err) => {
                println!("Could not expand icon file name: {}", err);
                "".to_string()
            }
        };
        g_object_unref(icon as *mut GObject);
    }
    result
}
