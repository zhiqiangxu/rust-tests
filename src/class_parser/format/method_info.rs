use super::attributes::Type;
#[derive(Debug)]
pub struct MethodInfo {
    pub acc_flags: u16,
    pub name_index: u16,
    pub desc_index: u16,
    pub attrs: Vec<Type>,
}
