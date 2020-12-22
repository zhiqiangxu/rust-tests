use super::attributes::Type;
use super::constant_pool;
use super::field_info::FieldInfo;
use super::method_info::MethodInfo;
use super::version::Version;
use std::sync::Arc;

#[derive(Debug)]
pub struct ClassFile {
    pub version: Version,
    pub cp: Arc<Vec<constant_pool::Type>>,
    pub acc_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attrs: Vec<Type>,
}
