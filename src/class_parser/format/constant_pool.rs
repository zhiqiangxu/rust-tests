use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Type {
    Nop,
    Class {
        name_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer {
        v: [u8; 4],
    },
    Float {
        v: [u8; 4],
    },
    Long {
        v: [u8; 8],
    },
    Double {
        v: [u8; 8],
    },
    NameAndType {
        name_index: u16,
        desc_index: u16,
    },
    Utf8 {
        bytes: Arc<Vec<u8>>,
    },
    MethodHandle {
        ref_kind: u8,
        ref_index: u16,
    },
    MethodType {
        desc_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Unknown,
}

#[derive(Clone, Copy)]
pub enum Tag {
    Class,
    FieldRef,
    MethodRef,
    InterfaceMethodRef,
    String,
    Integer,
    Float,
    Long,
    Double,
    NameAndType,
    Utf8,
    MethodHandle,
    MethodType,
    InvokeDynamic,
}

impl From<u8> for Tag {
    fn from(tag: u8) -> Self {
        match tag {
            7 => Tag::Class,
            9 => Tag::FieldRef,
            10 => Tag::MethodRef,
            11 => Tag::InterfaceMethodRef,
            8 => Tag::String,
            3 => Tag::Integer,
            4 => Tag::Float,
            5 => Tag::Long,
            6 => Tag::Double,
            12 => Tag::NameAndType,
            1 => Tag::Utf8,
            15 => Tag::MethodHandle,
            16 => Tag::MethodType,
            18 => Tag::InvokeDynamic,
            _ => unreachable!(),
        }
    }
}

pub fn get_utf8(cp: &Arc<Vec<Type>>, idx: usize) -> Option<Arc<Vec<u8>>> {
    match cp.get(idx) {
        Some(Type::Utf8 { bytes }) => Some(bytes.clone()),
        _ => None,
    }
}

pub fn get_class_name(cp: &Arc<Vec<Type>>, idx: usize) -> Option<Arc<Vec<u8>>> {
    match cp.get(idx) {
        Some(Type::Class { name_index }) => get_utf8(cp, *name_index as usize),
        _ => None,
    }
}
