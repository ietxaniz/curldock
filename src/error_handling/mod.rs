use anyhow::{Context, Result};

pub trait AddDebugInfo<T> {
    fn add_debug_info(self, context: Option<&str>, file: &str, line: u32) -> Result<T>;
}

impl<T, E> AddDebugInfo<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn add_debug_info(self, context: Option<&str>, file: &str, line: u32) -> Result<T> {
        self.map_err(Into::into).with_context(move || {
            if let Some(ctx) = context {
                format!("{}\n(at {}:{})", ctx, file, line)
            } else {
                format!("\n(at {}:{})", file, line)
            }
        })
    }
}


#[macro_export]
macro_rules! debug_err {
    ($result:expr) => {
        $crate::error_handling::AddDebugInfo::add_debug_info($result, None, file!(), line!())
    };
    ($result:expr, $msg:literal) => {
        $crate::error_handling::AddDebugInfo::add_debug_info($result, Some($msg), file!(), line!())
    };
    ($result:expr, $fmt:literal, $($arg:tt)*) => {
        $crate::error_handling::AddDebugInfo::add_debug_info($result, Some(&format!($fmt, $($arg)*)), file!(), line!())
    };
}