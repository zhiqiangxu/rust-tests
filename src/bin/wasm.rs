use wasmtime::{Engine, Extern, Func, Instance, Linker, Module, Store};

fn main() {
    let engine = Engine::default();

    let store = Store::new(&engine);

    let module = Module::from_file(&engine, "/Users/xuzhiqiang/Desktop/workspace/opensource/rust_exp/jvm/target//wasm32-unknown-unknown/debug/wasm_invoke.wasm").unwrap();
    let imports: Vec<_> = module.imports().collect();
    for import in imports {
        println!("module {:?}", import.module());
        println!("name {:?}", import.name());
    }

    let add = Func::wrap(&store, add);
    let instance = Instance::new(&store, &module, &[add.into()]).unwrap();
    let main = instance
        .get_func("invoke")
        .expect("`invoke` was not an exported function");
    main.call(&[]).unwrap();
}

pub fn add(a: i32, b: i32) -> i32 {
    println!("a+b={}", a + b);
    a + b
}
