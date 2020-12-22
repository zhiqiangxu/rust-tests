use std::sync::Arc;
use tracing::info;
#[derive(Debug)]
pub enum Type {
    ConstantValue {
        constant_value_index: u16,
    },
    Code(Code),
    StackMapTable {
        entries: Vec<StackMapFrame>,
    },
    Exceptions {
        exceptions: Vec<u16>,
    },
    InnerClasses {
        classes: Vec<InnerClass>,
    },
    EnclosingMethod {
        em: EnclosingMethod,
    },
    Synthetic,
    Signature {
        signature_index: u16,
    },
    SourceFile {
        source_file_index: u16,
    },
    SourceDebugExtension {
        debug_extension: Arc<Vec<u8>>,
    },
    LineNumberTable {
        tables: Vec<LineNumber>,
    },
    LocalVariableTable {
        tables: Vec<LocalVariable>,
    },
    LocalVariableTypeTable {
        tables: Vec<LocalVariable>,
    },
    Deprecated,
    RuntimeVisibleAnnotations {
        raw: Arc<Vec<u8>>,
        annotations: Vec<AnnotationEntry>,
    },
    RuntimeInvisibleAnnotations {
        raw: Arc<Vec<u8>>,
        annotations: Vec<AnnotationEntry>,
    },
    RuntimeVisibleParameterAnnotations {
        raw: Arc<Vec<u8>>,
        annotations: Vec<AnnotationEntry>,
    },
    RuntimeInvisibleParameterAnnotations {
        raw: Arc<Vec<u8>>,
        annotations: Vec<AnnotationEntry>,
    },
    RuntimeVisibleTypeAnnotations {
        raw: Arc<Vec<u8>>,
        annotations: Vec<TypeAnnotation>,
    },
    RuntimeInvisibleTypeAnnotations {
        raw: Arc<Vec<u8>>,
        annotations: Vec<TypeAnnotation>,
    },
    AnnotationDefault {
        raw: Arc<Vec<u8>>,
        default_value: ElementValueType,
    },
    BootstrapMethods {
        n: u16,
        methods: Vec<BootstrapMethod>,
    },
    MethodParameters {
        parameters: Vec<MethodParameter>,
    },
    Unknown,
}

#[derive(Clone, Copy)]
pub enum Tag {
    ConstantValue,
    Code,
    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile,
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    AnnotationDefault,
    BootstrapMethods,
    MethodParameters,
    Unknown,
}

impl From<&[u8]> for Tag {
    fn from(raw: &[u8]) -> Self {
        match raw {
            b"ConstantValue" => Tag::ConstantValue,
            b"Code" => Tag::Code,
            b"StackMapTable" => Tag::StackMapTable,
            b"Exceptions" => Tag::Exceptions,
            b"InnerClasses" => Tag::InnerClasses,
            b"EnclosingMethod" => Tag::EnclosingMethod,
            b"Synthetic" => Tag::Synthetic,
            b"Signature" => Tag::Signature,
            b"SourceFile" => Tag::SourceFile,
            b"SourceDebugExtension" => Tag::SourceDebugExtension,
            b"LineNumberTable" => Tag::LineNumberTable,
            b"LocalVariableTable" => Tag::LocalVariableTable,
            b"LocalVariableTypeTable" => Tag::LocalVariableTypeTable,
            b"Deprecated" => Tag::Deprecated,
            b"RuntimeVisibleAnnotations" => Tag::RuntimeVisibleAnnotations,
            b"RuntimeInvisibleAnnotations" => Tag::RuntimeInvisibleAnnotations,
            b"RuntimeVisibleParameterAnnotations" => Tag::RuntimeVisibleParameterAnnotations,
            b"RuntimeInvisibleParameterAnnotations" => Tag::RuntimeInvisibleParameterAnnotations,
            b"RuntimeVisibleTypeAnnotations" => Tag::RuntimeVisibleTypeAnnotations,
            b"RuntimeInvisibleTypeAnnotations" => Tag::RuntimeInvisibleTypeAnnotations,
            b"AnnotationDefault" => Tag::AnnotationDefault,
            b"BootstrapMethods" => Tag::BootstrapMethods,
            b"MethodParameters" => Tag::MethodParameters,
            _ => {
                info!("Unknown attr {}", unsafe {
                    std::str::from_utf8_unchecked(raw)
                });
                // error!("Unknown attr {}", String::from_utf8_lossy(raw));
                Tag::Unknown
            }
        }
    }
}
#[derive(Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Arc<Vec<u8>>,
    pub exceptions: Vec<CodeException>,
    pub attrs: Vec<Type>,
}

#[derive(Debug)]
pub struct CodeException {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}
#[derive(Debug)]
pub enum StackMapFrame {
    Same {
        tag: u8,
        offset_delta: u16,
    },
    SameLocals1StackItem {
        tag: u8,
        offset_delta: u16,
        stack: [VerificationTypeInfo; 1],
    },
    SameLocals1StackItemExtended {
        tag: u8,
        offset_delta: u16,
        stack: [VerificationTypeInfo; 1],
    },
    Chop {
        tag: u8,
        offset_delta: u16,
    },
    SameExtended {
        tag: u8,
        offset_delta: u16,
    },
    Append {
        tag: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    Full {
        tag: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
    Reserved(u8),
}

#[derive(Debug)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

#[derive(Debug)]
pub struct EnclosingMethod {
    pub class_index: u16,
    pub method_index: u16,
}

#[derive(Debug)]
pub struct LineNumber {
    pub start_pc: u16,
    pub number: u16,
}

#[derive(Debug)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Debug)]
pub struct AnnotationEntry {
    pub type_name: Arc<Vec<u8>>,
    pub pairs: Vec<ElementValuePair>,
}

#[derive(Debug)]
pub struct ElementValuePair {
    pub name_index: u16,
    pub value: ElementValueType,
}

#[derive(Debug)]
pub enum ElementValueType {
    Byte { val_index: u16 },
    Char { val_index: u16 },
    Double { val_index: u16 },
    Float { val_index: u16 },
    Int { val_index: u16 },
    Long { val_index: u16 },
    Short { val_index: u16 },
    Boolean { val_index: u16 },
    String { val_index: u16 },
    Enum { type_index: u16, val_index: u16 },
    Class { index: u16 },
    Annotation(AnnotationElementValue),
    Array { values: Vec<ElementValueType> },
    Unknown,
}
#[derive(Debug)]
pub struct AnnotationElementValue {
    pub value: AnnotationEntry,
}

#[derive(Debug)]
pub struct MethodParameter {
    pub name_index: u16,
    pub acc_flags: u16,
}

#[derive(Debug)]
pub struct BootstrapMethod {
    pub method_ref: u16,
    pub args: Vec<u16>,
}

#[derive(Debug)]
pub struct TypeAnnotation {
    pub target_info: TargetInfo,
    pub target_path: Vec<TypePath>,
    pub type_index: u16,
    pub pairs: Vec<ElementValuePair>,
}

#[derive(Debug)]
pub enum TargetInfo {
    TypeParameter {
        type_parameter_index: u8,
    },
    SuperType {
        supertype_index: u16,
    },
    TypeParameterBound {
        type_parameter_index: u8,
        bound_index: u8,
    },
    Empty,
    FormalParameter {
        formal_parameter_index: u8,
    },
    Throws {
        throws_type_index: u16,
    },
    LocalVar {
        table: Vec<LocalVarTargetTable>,
    },
    Catch {
        exception_table_index: u16,
    },
    Offset {
        offset: u16,
    },
    TypeArgument {
        offset: u16,
        type_argument_index: u8,
    },
}

#[derive(Debug)]
pub struct LocalVarTargetTable {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}
#[derive(Debug)]
pub struct TypePath {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}
#[derive(Debug)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized { offset: u16 },
}

#[derive(Copy, Clone)]
pub enum ElementValueTag {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    String,
    Enum,
    Class,
    Annotation,
    Array,
    Unknown,
}

impl From<u8> for ElementValueTag {
    fn from(v: u8) -> Self {
        match v {
            b'B' => ElementValueTag::Byte,
            b'C' => ElementValueTag::Char,
            b'D' => ElementValueTag::Double,
            b'F' => ElementValueTag::Float,
            b'I' => ElementValueTag::Int,
            b'J' => ElementValueTag::Long,
            b'S' => ElementValueTag::Short,
            b'Z' => ElementValueTag::Boolean,
            b's' => ElementValueTag::String,
            b'e' => ElementValueTag::Enum,
            b'c' => ElementValueTag::Class,
            b'@' => ElementValueTag::Annotation,
            b'[' => ElementValueTag::Array,
            _ => ElementValueTag::Unknown,
        }
    }
}
