use super::CONFIG;

#[inline]
pub fn file_path(relative_path: &str) -> String {
    format!("{}{}", &CONFIG.root, relative_path)
}
