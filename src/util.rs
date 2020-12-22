pub const FILE_SEP: &str = platform::FILE_SEP;
pub const PATH_SEP: &str = platform::PATH_SEP;

#[cfg(unix)]
mod platform {
    pub const FILE_SEP: &str = "/";
    pub const PATH_SEP: &str = ":";
}

#[cfg(windows)]
mod platform {
    pub const FILE_SEP: &str = "\\";
    pub const PATH_SEP: &str = ";";
}

#[macro_export]
macro_rules! def_ref {
    ($name:ident, $t:ty) => {
        pub type $name = std::sync::Arc<Box<$t>>;
    };
}
