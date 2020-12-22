use clap::{App, Arg};
use jvm::{class_loader::ClassLoader, class_path_manager::ClassPathManager};
use std::sync::Arc;

fn main() {
    let matches = App::new("java programme")
        .version("1.0")
        .author("zhiqiangxu")
        .arg(
            Arg::new("cp")
                .long("cp")
                .about("class search path of directories and zip/jar files")
                .takes_value(true),
        )
        .arg(Arg::new("main").required(true))
        .get_matches();

    let cp = matches.value_of("cp");
    let main = matches.value_of("main").unwrap();

    let mut cpm = ClassPathManager::new();
    if let Some(class_path) = cp {
        cpm.add_class_paths(class_path).unwrap();
        let cl = ClassLoader::new(Arc::new(cpm), None);

        let cf = cl.load_class(main).unwrap();
        println!("{:?}", cf);
    }
}
