#![feature(test)]

#[macro_use]
pub mod util;

pub mod class_loader;
pub mod class_parser;
pub mod class_path_manager;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use std::sync::{Arc, Mutex};
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_class_path_manager_and_class_loader() {
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        let cp = format!(
            "{}{}{}{}{}",
            cargo_dir,
            util::FILE_SEP,
            "resource",
            util::FILE_SEP,
            "test",
        );

        println!("cp: {}", cp);
        let mut cpm = class_path_manager::ClassPathManager::new();

        {
            let result = cpm.add_class_path(&cp);
            assert!(result.is_ok(), "error: {}", result.err().unwrap());
        }

        let hello_world = "HelloWorld";
        let hello_world2 = "HelloWorld2";

        assert!(cpm.search_class(hello_world).is_ok());
        assert!(cpm.search_class(hello_world2).is_err());

        let cpm_ref = Arc::new(cpm);
        let cl = class_loader::ClassLoader::new(cpm_ref, None);
        assert!(cl.load_class(hello_world).is_some());
        assert!(cl.load_class(hello_world2).is_none());
    }
    #[bench]
    fn bench_mutex(b: &mut Bencher) {
        let m = Mutex::new(0);

        b.iter(|| drop(m.lock().unwrap()))
    }
}
