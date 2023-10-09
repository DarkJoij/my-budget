pub mod config;
pub mod store;

use iced::Theme;

pub trait Names {
    fn sys_name(&self) -> &str {
        unimplemented!()
    }

    fn display_name(&self) -> &str {
        unimplemented!()
    }
}

impl Names for Theme {
    fn sys_name(&self) -> &str {
        if matches!(self, Theme::Light) { "Light" } else { "Dark" }
    }
}

pub trait ToClear<T> {
    fn to_clear(&self) -> T;
}

impl ToClear<Theme> for String {
    fn to_clear(&self) -> Theme {
        if self.as_str() == "Light" { Theme::Light } else { Theme::Dark }
    }
}
