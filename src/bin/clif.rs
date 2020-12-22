use cranelift::prelude::*;
use cranelift_codegen::binemit::NullTrapSink;
use cranelift_codegen::ir::types::*;
use cranelift_codegen::ir::Function;
use cranelift_module::{default_libcall_names, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};
use std::mem;
use std::str::FromStr;
use wasmtime_jit::native;

fn main() {
    let target = native::builder().finish(settings::Flags::new(settings::builder()));

    let mut sig = Signature::new(target.default_call_conv());
    sig.returns.push(AbiParam::new(I32));
    sig.params.push(AbiParam::new(I32));
    let mut fn_ctx = FunctionBuilderContext::new();
    let mut func = Function::with_name_signature(ExternalName::testcase("sample"), sig);
    {
        let lazy_seal = false;
        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        let block3 = builder.create_block();
        let x = Variable::new(0);
        let y = Variable::new(1);
        let z = Variable::new(2);
        builder.declare_var(x, I32);
        builder.declare_var(y, I32);
        builder.declare_var(z, I32);
        builder.append_block_params_for_function_params(block0);

        builder.switch_to_block(block0);
        if !lazy_seal {
            builder.seal_block(block0);
        }
        {
            let tmp = builder.block_params(block0)[0]; // the first function parameter
            builder.def_var(x, tmp);
        }
        {
            let tmp = builder.ins().iconst(I32, 2);
            builder.def_var(y, tmp);
        }
        {
            let arg1 = builder.use_var(x);
            let arg2 = builder.use_var(y);
            let tmp = builder.ins().iadd(arg1, arg2);
            builder.def_var(z, tmp);
        }
        builder.ins().jump(block1, &[]);

        builder.switch_to_block(block1);
        {
            let arg1 = builder.use_var(y);
            let arg2 = builder.use_var(z);
            let tmp = builder.ins().iadd(arg1, arg2);
            builder.def_var(z, tmp);
        }
        {
            let arg = builder.use_var(y);
            builder.ins().brnz(arg, block3, &[]);
        }
        builder.ins().jump(block2, &[]);

        builder.switch_to_block(block2);
        if !lazy_seal {
            builder.seal_block(block2);
        }
        {
            let arg1 = builder.use_var(z);
            let arg2 = builder.use_var(x);
            let tmp = builder.ins().isub(arg1, arg2);
            builder.def_var(z, tmp);
        }
        {
            let arg = builder.use_var(y);
            builder.ins().return_(&[arg]);
        }

        builder.switch_to_block(block3);
        if !lazy_seal {
            builder.seal_block(block3);
        }

        {
            let arg1 = builder.use_var(y);
            let arg2 = builder.use_var(x);
            let tmp = builder.ins().isub(arg1, arg2);
            builder.def_var(y, tmp);
        }
        builder.ins().jump(block1, &[]);
        if !lazy_seal {
            builder.seal_block(block1);
        }

        if lazy_seal {
            builder.seal_all_blocks();
        }

        builder.finalize();
    }

    println!("{}", func.display(None).to_string())
}
