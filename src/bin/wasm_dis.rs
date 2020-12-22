use cranelift_wasm::translate_module;
// use wasmtime_cranelift::FuncEnvironment;
use wasmtime::{Config, Engine};

use cranelift_codegen::settings::Configurable;
use std::cmp;
use wasmtime_environ::{settings, CacheConfig, ModuleEnvironment, Tunables};
use wasmtime_jit::{native, CompilationStrategy, Compiler};
use wat;

fn main() {
    let mut tunables = Tunables::default();
    if cfg!(windows) {
        // For now, use a smaller footprint on Windows so that we don't
        // don't outstrip the paging file.
        tunables.static_memory_bound = cmp::min(tunables.static_memory_bound, 0x100);
        tunables.static_memory_offset_guard_size =
            cmp::min(tunables.static_memory_offset_guard_size, 0x10000);
    }

    let mut flags = settings::builder();

    // There are two possible traps for division, and this way
    // we get the proper one if code traps.
    flags
        .enable("avoid_div_traps")
        .expect("should be valid flag");

    // Invert cranelift's default-on verification to instead default off.
    flags
        .set("enable_verifier", "false")
        .expect("should be valid flag");

    // Turn on cranelift speed optimizations by default
    flags
        .set("opt_level", "speed")
        .expect("should be valid flag");

    // We don't use probestack as a stack limit mechanism
    flags
        .set("enable_probestack", "false")
        .expect("should be valid flag");

    let isa = native::builder().finish(settings::Flags::new(flags));
    let strategy = CompilationStrategy::Auto;
    let cache_config = CacheConfig::new_cache_disabled();

    let compiler = Compiler::new(isa, strategy, cache_config, tunables);

    let data = wat::parse_str(
        r#"
        (module $module_name
            (func $func_name (local $loc_name i32)
            )
        )"#,
    )
    .unwrap();

    let environ = ModuleEnvironment::new(compiler.frontend_config(), compiler.tunables());
    let translation = environ.translate(data.as_ref()).unwrap();
}
