use nom::{
    call, count, do_parse, named, named_args,
    number::streaming::{be_u16, be_u32, be_u8},
    peek, switch, tag, take, value,
};

use super::format::{
    attributes::{
        AnnotationElementValue, AnnotationEntry, BootstrapMethod, Code, CodeException,
        ElementValuePair, ElementValueTag, ElementValueType, EnclosingMethod, InnerClass,
        LineNumber, LocalVarTargetTable, LocalVariable, MethodParameter, StackMapFrame,
        Tag as AttrTag, TargetInfo, Type as AttributeType, TypeAnnotation, TypePath,
        VerificationTypeInfo,
    },
    class_file::ClassFile,
    constant_pool,
    field_info::FieldInfo,
    method_info::MethodInfo,
    version::Version,
};

use std::sync::Arc;

named!(
    version<Version>,
    do_parse!(minor: be_u16 >> major: be_u16 >> (Version { minor, major }))
);

named!(
    constant_tag<constant_pool::Tag>,
    do_parse!(tag: be_u8 >> (constant_pool::Tag::from(tag)))
);

fn constant_pool(input: &[u8]) -> nom::IResult<&[u8], Arc<Vec<constant_pool::Type>>> {
    let (mut input, count) = be_u16(input)?;

    let mut output = Vec::with_capacity(count as usize);
    output.push(constant_pool::Type::Nop);

    let mut i = 1;
    while i < count {
        let (new_input, constant_type) = cp_entry(input)?;
        input = new_input;

        i += 1;
        output.push(constant_type.clone());

        //spec 4.4.5
        match constant_type {
            constant_pool::Type::Long { .. } | constant_pool::Type::Double { .. } => {
                i += 1;
                output.push(constant_pool::Type::Nop);
            }
            _ => (),
        }
    }

    Ok((input, Arc::new(output)))
}

// Const generics still not in stable,
// idk how to write this fancier without them D:
// Hope compiler will rewrite this properly
macro_rules! gen_take_exact {
    ($count: expr, $name: ident) => {
        fn $name(input: &[u8]) -> nom::IResult<&[u8], [u8; $count]> {
            let mut output = [0; $count];
            // TODO: Nom error
            assert!(input.len() >= $count);
            for i in 0..$count {
                output[i] = input[i];
            }
            Ok((&input[$count..], output))
        }
    };
}

gen_take_exact!(4, take_exact_4);
gen_take_exact!(8, take_exact_8);

named!(
    cp_entry<constant_pool::Type>,
    do_parse!(
        ct: constant_tag
            >> entry:
                switch!(value!(ct),
                    constant_pool::Tag::Class => do_parse!(
                        name_index: be_u16 >>
                        (constant_pool::Type::Class { name_index })
                    ) |
                    constant_pool::Tag::FieldRef => do_parse!(
                        class_index: be_u16 >>
                        name_and_type_index: be_u16 >>
                        (constant_pool::Type::FieldRef { class_index, name_and_type_index })
                    ) |
                    constant_pool::Tag::MethodRef => do_parse!(
                        class_index: be_u16 >>
                        name_and_type_index: be_u16 >>
                        (constant_pool::Type::MethodRef { class_index, name_and_type_index })
                    ) |
                    constant_pool::Tag::InterfaceMethodRef => do_parse!(
                        class_index: be_u16 >>
                        name_and_type_index: be_u16 >>
                        (constant_pool::Type::InterfaceMethodRef { class_index, name_and_type_index })
                    ) |
                    constant_pool::Tag::String => do_parse!(
                        string_index: be_u16 >>
                        (constant_pool::Type::String { string_index })
                    ) |
                    constant_pool::Tag::Integer => do_parse!(
                        v: take_exact_4 >>
                        (constant_pool::Type::Integer { v })
                    ) |
                    constant_pool::Tag::Float => do_parse!(
                        v: take_exact_4 >>
                        (constant_pool::Type::Float { v })
                    ) |
                    constant_pool::Tag::Long => do_parse!(
                        v: take_exact_8 >>
                        (constant_pool::Type::Long { v })
                    ) |
                    constant_pool::Tag::Double => do_parse!(
                        v: take_exact_8 >>
                        (constant_pool::Type::Double { v })
                    ) |
                    constant_pool::Tag::NameAndType => do_parse!(
                        name_index: be_u16 >>
                        desc_index: be_u16 >>
                        (constant_pool::Type::NameAndType { name_index, desc_index })
                    ) |
                    constant_pool::Tag::Utf8 => do_parse!(
                        length: be_u16 >>
                        bytes: take!(length) >>
                        (constant_pool::Type::Utf8 { bytes: Arc::new(Vec::from(bytes)) })
                    ) |
                    constant_pool::Tag::MethodHandle => do_parse!(
                        ref_kind: be_u8 >>
                        ref_index: be_u16 >>
                        (constant_pool::Type::MethodHandle { ref_kind, ref_index })
                    ) |
                    constant_pool::Tag::MethodType => do_parse!(
                        desc_index: be_u16 >>
                        (constant_pool::Type::MethodType { desc_index })
                    ) |
                    constant_pool::Tag::InvokeDynamic => do_parse!(
                        bootstrap_method_attr_index: be_u16 >>
                        name_and_type_index: be_u16 >>
                        (constant_pool::Type::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index })
                    )
                )
            >> (entry)
    )
);

named!(
    verification_type_info<VerificationTypeInfo>,
    do_parse!(
        id: be_u8
            >> inner:
                switch!(value!(id),
                    0 => value!(VerificationTypeInfo::Top) |
                    1 => value!(VerificationTypeInfo::Integer) |
                    2 => value!(VerificationTypeInfo::Float) |
                    3 => value!(VerificationTypeInfo::Long) |
                    4 => value!(VerificationTypeInfo::Double) |
                    5 => value!(VerificationTypeInfo::Null) |
                    6 => value!(VerificationTypeInfo::UninitializedThis) |
                    7 => do_parse!(
                        cpool_index: be_u16 >>
                        (VerificationTypeInfo::Object {cpool_index})
                    ) |
                    8 => do_parse!(
                        offset: be_u16 >>
                        (VerificationTypeInfo::Uninitialized {offset})
                    )
                )
            >> (inner)
    )
);

named!(
    stack_map_frame<StackMapFrame>,
    do_parse!(
        frame_type: be_u8
            >> inner:
                switch!(value!(frame_type),
                    0..=63 => value!(StackMapFrame::Same {tag: frame_type, offset_delta: frame_type as u16}) |
                    64..=127 => do_parse!(
                        offset_delta: value!((frame_type-64) as u16) >>
                        type_info: verification_type_info >>
                        (StackMapFrame::SameLocals1StackItem {
                            tag: frame_type,
                            offset_delta,
                            stack: [type_info],
                        })
                    ) |
                    128..=246 => value!(StackMapFrame::Reserved(frame_type)) |
                    247 => do_parse!(
                        offset_delta: be_u16 >>
                        type_info: verification_type_info >>
                        (StackMapFrame::SameLocals1StackItem {
                            tag: frame_type,
                            offset_delta,
                            stack: [type_info],
                        })
                    ) |
                    248..=250 => do_parse!(
                        offset_delta: be_u16 >>
                        (StackMapFrame::Chop {
                            tag: frame_type,
                            offset_delta,
                        })
                    ) |
                    251 => do_parse!(
                        offset_delta: be_u16 >>
                        (StackMapFrame::SameExtended {
                            tag: frame_type,
                            offset_delta
                        })
                    ) |
                    252..=254 => do_parse!(
                        offset_delta: be_u16 >>
                        locals_count: value!(frame_type - 251) >>
                        locals: count!(verification_type_info, locals_count as usize) >>
                        (StackMapFrame::Append {
                            tag: frame_type,
                            offset_delta,
                            locals,
                        })
                    ) |
                    255 => do_parse!(
                        offset_delta: be_u16 >>
                        locals_count: be_u16 >>
                        locals: count!(verification_type_info, locals_count as usize) >>
                        stack_count: be_u16 >>
                        stack: count!(verification_type_info, stack_count as usize) >>
                        (StackMapFrame::Full {
                            tag: frame_type,
                            offset_delta,
                            locals,
                            stack,
                        })
                    )
                )
            >> (inner)
    )
);

named!(
    inner_class<InnerClass>,
    do_parse!(
        inner_class_info_index: be_u16
            >> outer_class_info_index: be_u16
            >> inner_name_index: be_u16
            >> inner_class_access_flags: be_u16
            >> (InnerClass {
                inner_class_info_index,
                outer_class_info_index,
                inner_name_index,
                inner_class_access_flags,
            })
    )
);

named!(
    enclosing_method<EnclosingMethod>,
    do_parse!(
        class_index: be_u16
            >> method_index: be_u16
            >> (EnclosingMethod {
                class_index,
                method_index,
            })
    )
);

named!(
    line_number<LineNumber>,
    do_parse!(start_pc: be_u16 >> number: be_u16 >> (LineNumber { start_pc, number }))
);

named!(
    local_variable<LocalVariable>,
    do_parse!(
        start_pc: be_u16
            >> length: be_u16
            >> name_index: be_u16
            >> signature_index: be_u16
            >> index: be_u16
            >> (LocalVariable {
                start_pc,
                length,
                name_index,
                signature_index,
                index,
            })
    )
);

named!(
    element_value_tag<ElementValueTag>,
    do_parse!(tag: be_u8 >> (ElementValueTag::from(tag)))
);

// I didn't found a way to turn byte/char/double/float/... boilerplate into a macro(
named_args!(element_value_type(cp: Arc<Vec<constant_pool::Type>>)<ElementValueType>, do_parse!(
    tag: element_value_tag >>
    inner: switch!(value!(tag),
        ElementValueTag::Byte => do_parse!(
            val_index: be_u16 >>
            (ElementValueType::Byte {val_index})
        ) |
        ElementValueTag::Char => do_parse!(
            val_index: be_u16 >>
            (ElementValueType::Char {val_index})
        ) |
        ElementValueTag::Double => do_parse!(
            val_index: be_u16 >>
            (ElementValueType::Double {val_index})
        ) |
        ElementValueTag::Float => do_parse!(
            val_index: be_u16 >>
            (ElementValueType::Float {val_index})
        ) |
        ElementValueTag::Byte => do_parse!(
            val_index: be_u16 >>
            (ElementValueType::Byte {val_index})
        ) |
        ElementValueTag::Int => do_parse!(
            val_index: be_u16 >>
            (ElementValueType::Int {val_index})
        ) |
        ElementValueTag::Long => do_parse!(
            val_index: be_u16 >>
            (ElementValueType::Long {val_index})
        ) |
        ElementValueTag::Short => do_parse!(
            val_index: be_u16 >>
            (ElementValueType::Short {val_index})
        ) |
        ElementValueTag::Boolean => do_parse!(
            val_index: be_u16 >>
            (ElementValueType::Boolean {val_index})
        ) |
        ElementValueTag::String => do_parse!(
            val_index: be_u16 >>
            (ElementValueType::String {val_index})
        ) |
        ElementValueTag::Enum => do_parse!(
            type_index: be_u16 >>
            val_index: be_u16 >>
            (ElementValueType::Enum {type_index, val_index})
        ) |
        ElementValueTag::Class => do_parse!(
            index: be_u16 >>
            (ElementValueType::Class {index})
        ) |
        ElementValueTag::Annotation => do_parse!(
            value: call!(annotation_entry, cp) >>
            (ElementValueType::Annotation(AnnotationElementValue {value}))
        ) |
        ElementValueTag::Array => do_parse!(
            array_size: be_u16 >>
            values: count!(call!(element_value_type, cp.clone()), array_size as usize) >>
            (ElementValueType::Array {
                values,
            })
        ) |
        ElementValueTag::Unknown => value!(ElementValueType::Unknown)
    ) >>
    (inner)
));

named_args!(element_value_pair(cp: Arc<Vec<constant_pool::Type>>)<ElementValuePair>, do_parse!(
    name_index: be_u16 >>
    value: call!(element_value_type, cp) >>
    (ElementValuePair {name_index, value})
));

named_args!(annotation_entry(cp: Arc<Vec<constant_pool::Type>>)<AnnotationEntry>, do_parse!(
    type_index: be_u16 >>
    pair_count: be_u16 >>
    pairs: count!(call!(element_value_pair, cp.clone()), pair_count as usize) >>
    type_name: value!(constant_pool::get_utf8(&cp, type_index as usize).expect("Missing type name")) >>
    (AnnotationEntry {type_name, pairs})
));

named!(
    local_var_target_table<LocalVarTargetTable>,
    do_parse!(
        start_pc: be_u16
            >> length: be_u16
            >> index: be_u16
            >> (LocalVarTargetTable {
                start_pc,
                length,
                index
            })
    )
);

named!(
    target_info<TargetInfo>,
    do_parse!(
        target_type: be_u8
            >> inner:
                switch!(value!(target_type),
                    0x00 | 0x01 => do_parse!(
                        type_parameter_index: be_u8 >>
                        (TargetInfo::TypeParameter { type_parameter_index })
                    ) |
                    0x10 => do_parse!(
                        supertype_index: be_u16 >>
                        (TargetInfo::SuperType { supertype_index })
                    ) |
                    0x11 | 0x12 => do_parse!(
                        type_parameter_index: be_u8 >>
                        bound_index: be_u8 >>
                        (TargetInfo::TypeParameterBound {type_parameter_index, bound_index})
                    ) |
                    0x13 | 0x14 | 0x15 => value!(TargetInfo::Empty) |
                    0x16 => do_parse!(
                        formal_parameter_index: be_u8 >>
                        (TargetInfo::FormalParameter {formal_parameter_index})
                    ) |
                    0x17 => do_parse!(
                        throws_type_index: be_u16 >>
                        (TargetInfo::Throws {throws_type_index})
                    ) |
                    0x40 | 0x41 => do_parse!(
                        item_count: be_u16 >>
                        items: count!(local_var_target_table, item_count as usize) >>
                        (TargetInfo::LocalVar {table: items})
                    ) |
                    0x42 => do_parse!(
                        exception_table_index: be_u16 >>
                        (TargetInfo::Catch {exception_table_index})
                    ) |
                    0x43 | 0x44 | 0x45 | 0x46 => do_parse!(
                        offset: be_u16 >>
                        (TargetInfo::Offset {offset})
                    ) |
                    0x47 | 0x48 | 0x49 | 0x4A | 0x4B => do_parse!(
                        offset: be_u16 >>
                        type_argument_index: be_u8 >>
                        (TargetInfo::TypeArgument {offset, type_argument_index})
                    )
                )
            >> (inner)
    )
);

named!(
    type_path<TypePath>,
    do_parse!(
        type_path_kind: be_u8
            >> type_argument_index: be_u8
            >> (TypePath {
                type_path_kind,
                type_argument_index,
            })
    )
);

named_args!(type_annotation(cp: Arc<Vec<constant_pool::Type>>)<TypeAnnotation>, do_parse!(
    target_info: target_info >>
    target_path_part_count: be_u8 >>
    target_path: count!(type_path, target_path_part_count as usize) >>
    type_index: be_u16 >>
    pair_count: be_u16 >>
    pairs: count!(call!(element_value_pair, cp.clone()), pair_count as usize) >>
    (TypeAnnotation {
        target_info,
        target_path,
        type_index,
        pairs,
    })
));

named!(
    bootstrap_method<BootstrapMethod>,
    do_parse!(
        method_ref: be_u16
            >> arg_count: be_u16
            >> args: count!(be_u16, arg_count as usize)
            >> (BootstrapMethod { method_ref, args })
    )
);

named!(
    method_parameter<MethodParameter>,
    do_parse!(
        name_index: be_u16
            >> acc_flags: be_u16
            >> (MethodParameter {
                name_index,
                acc_flags
            })
    )
);

named!(
    code_exception<CodeException>,
    do_parse!(
        start_pc: be_u16
            >> end_pc: be_u16
            >> handler_pc: be_u16
            >> catch_type: be_u16
            >> (CodeException {
                start_pc,
                end_pc,
                handler_pc,
                catch_type
            })
    )
);

named_args!(attr_type_vec(cp: Arc<Vec<constant_pool::Type>>)<Vec<AttributeType>>, do_parse!(
    attrs_count: be_u16 >>
    attrs: count!(call!(attr_type, cp.clone()), attrs_count as usize) >>
    (attrs)
));

named_args!(attr_type(cp: Arc<Vec<constant_pool::Type>>)<AttributeType>, do_parse!(
    tag: call!(attr_tag, cp.clone()) >>
    length: be_u32 >>
    attr: call!(attr_sized, tag, length as usize, cp) >>
    (attr)
));

named_args!(attr_tag(cp: Arc<Vec<constant_pool::Type>>)<AttrTag>, do_parse!(
    name_index: be_u16 >>
    name: value!(constant_pool::get_utf8(&cp, name_index as usize).expect("Missing name")) >>
    inner: value!(AttrTag::from(name.as_slice())) >>
    (inner)
));

named_args!(attr_sized(tag: AttrTag, self_len: usize, cp: Arc<Vec<constant_pool::Type>>)<AttributeType>, switch!(value!(tag),
    AttrTag::ConstantValue => do_parse!(
        constant_value_index: be_u16 >>
        (AttributeType::ConstantValue {constant_value_index})
    ) |
    AttrTag::Code => do_parse!(
        max_stack: be_u16 >>
        max_locals: be_u16 >>
        len: be_u32 >>
        code: take!(len) >> // TODO: Parse code in same time?)
        exception_count: be_u16 >>
        exceptions: count!(code_exception, exception_count as usize) >>
        attrs: call!(attr_type_vec, cp) >>
        (AttributeType::Code(Code {
            max_stack,
            max_locals,
            code: Arc::new(Vec::from(code)),
            exceptions,
            attrs,
        }))
    ) |
    AttrTag::StackMapTable => do_parse!(
        frame_count: be_u16 >>
        frames: count!(stack_map_frame, frame_count as usize) >>
        (AttributeType::StackMapTable { entries: frames })
    ) |
    AttrTag::Exceptions => do_parse!(
        exception_count: be_u16 >>
        exceptions: count!(be_u16, exception_count as usize) >>
        (AttributeType::Exceptions { exceptions })
    ) |
    AttrTag::InnerClasses => do_parse!(
        class_count: be_u16 >>
        classes: count!(inner_class, class_count as usize) >>
        (AttributeType::InnerClasses { classes })
    ) |
    AttrTag::EnclosingMethod => do_parse!(
        em: enclosing_method >>
        (AttributeType::EnclosingMethod { em })
    ) |
    AttrTag::Synthetic => value!(AttributeType::Synthetic) |
    AttrTag::Signature => do_parse!(
        signature_index: be_u16 >>
        (AttributeType::Signature { signature_index })
    ) |
    AttrTag::SourceFile => do_parse!(
        source_file_index: be_u16 >>
        (AttributeType::SourceFile { source_file_index })
    ) |
    AttrTag::SourceDebugExtension => do_parse!(
        debug_extension: take!(self_len) >>
        (AttributeType::SourceDebugExtension { debug_extension: Arc::new(Vec::from(debug_extension)) })
    ) |
    AttrTag::LineNumberTable => do_parse!(
        line_count: be_u16 >>
        lines: count!(line_number, line_count as usize) >>
        (AttributeType::LineNumberTable { tables: lines })
    ) |
    AttrTag::LocalVariableTable => do_parse!(
        variable_count: be_u16 >>
        variables: count!(local_variable, variable_count as usize) >>
        (AttributeType::LocalVariableTable { tables: variables })
    ) |
    AttrTag::LocalVariableTypeTable => do_parse!(
        variable_count: be_u16 >>
        variables: count!(local_variable, variable_count as usize) >>
        (AttributeType::LocalVariableTypeTable { tables: variables })
    ) |
    AttrTag::Deprecated => value!(AttributeType::Deprecated) |
    AttrTag::RuntimeVisibleAnnotations => do_parse!(
        raw: peek!(take!(self_len)) >>
        annotation_count: be_u16 >>
        annotations: count!(call!(annotation_entry, cp.clone()), annotation_count as usize) >>
        (AttributeType::RuntimeVisibleAnnotations {raw: Arc::new(Vec::from(raw)), annotations})
    ) |
    AttrTag::RuntimeInvisibleAnnotations => do_parse!(
        raw: peek!(take!(self_len)) >>
        annotation_count: be_u16 >>
        annotations: count!(call!(annotation_entry, cp.clone()), annotation_count as usize) >>
        (AttributeType::RuntimeInvisibleAnnotations {raw: Arc::new(Vec::from(raw)), annotations})
    ) |
    AttrTag::RuntimeVisibleParameterAnnotations => do_parse!(
        raw: peek!(take!(self_len)) >>
        annotation_count: be_u16 >>
        annotations: count!(call!(annotation_entry, cp.clone()), annotation_count as usize) >>
        (AttributeType::RuntimeVisibleParameterAnnotations {raw: Arc::new(Vec::from(raw)), annotations})
    ) |
    AttrTag::RuntimeInvisibleParameterAnnotations => do_parse!(
        raw: peek!(take!(self_len)) >>
        annotation_count: be_u16 >>
        annotations: count!(call!(annotation_entry, cp.clone()), annotation_count as usize) >>
        (AttributeType::RuntimeInvisibleParameterAnnotations {raw: Arc::new(Vec::from(raw)), annotations})
    ) |
    AttrTag::RuntimeVisibleTypeAnnotations => do_parse!(
        raw: peek!(take!(self_len)) >>
        annotation_count: be_u16 >>
        annotations: count!(call!(type_annotation, cp.clone()), annotation_count as usize) >>
        (AttributeType::RuntimeVisibleTypeAnnotations {raw: Arc::new(Vec::from(raw)), annotations})
    ) |
    AttrTag::RuntimeInvisibleTypeAnnotations => do_parse!(
        raw: peek!(take!(self_len)) >>
        annotation_count: be_u16 >>
        annotations: count!(call!(type_annotation, cp.clone()), annotation_count as usize) >>
        (AttributeType::RuntimeInvisibleTypeAnnotations {raw: Arc::new(Vec::from(raw)), annotations})
    ) |
    AttrTag::AnnotationDefault => do_parse!(
        raw: peek!(take!(self_len)) >>
        default_value: call!(element_value_type, cp) >>
        (AttributeType::AnnotationDefault {raw: Arc::new(Vec::from(raw)), default_value})
    ) |
    AttrTag::BootstrapMethods => do_parse!(
        method_count: be_u16 >>
        methods: count!(bootstrap_method, method_count as usize) >>
        (AttributeType::BootstrapMethods {n:method_count, methods})
    ) |
    AttrTag::MethodParameters => do_parse!(
        parameter_count: be_u8 >>
        parameters: count!(method_parameter, parameter_count as usize) >>
        (AttributeType::MethodParameters {parameters})
    ) |
    AttrTag::Unknown => do_parse!(
        _data: take!(self_len) >>
        (AttributeType::Unknown)
    )
));

named_args!(field(cp: Arc<Vec<constant_pool::Type>>)<FieldInfo>, do_parse!(
    acc_flags: be_u16 >>
    name_index: be_u16 >>
    desc_index: be_u16 >>
    attrs: call!(attr_type_vec, cp) >>
    (FieldInfo {
        acc_flags,
        name_index,
        desc_index,
        attrs,
    })
));

named_args!(method_info(cp: Arc<Vec<constant_pool::Type>>)<MethodInfo>, do_parse!(
    acc_flags: be_u16 >>
    name_index: be_u16 >>
    desc_index: be_u16 >>
    attrs: call!(attr_type_vec, cp) >>
    (MethodInfo {
        acc_flags,
        name_index,
        desc_index,
        attrs,
    })
));

named!(
    class_file<ClassFile>,
    do_parse!(
        _magic: tag!(b"\xCA\xFE\xBA\xBE")
            >> version: version
            >> cp: constant_pool
            >> acc_flags: be_u16
            >> this_class: be_u16
            >> super_class: be_u16
            >> interfaces_count: be_u16
            >> interfaces: count!(be_u16, interfaces_count as usize)
            >> fields_count: be_u16
            >> fields: count!(call!(field, cp.clone()), fields_count as usize)
            >> method_count: be_u16
            >> methods: count!(call!(method_info, cp.clone()), method_count as usize)
            >> attrs: call!(attr_type_vec, cp.clone())
            >> (ClassFile {
                version,
                cp: cp.clone(),
                acc_flags,
                this_class,
                super_class,
                interfaces,
                fields,
                methods,
                attrs
            })
    )
);

pub fn parse(input: &[u8]) -> nom::IResult<&[u8], ClassFile> {
    class_file(input)
}
