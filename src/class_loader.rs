use super::class_parser::format::class_file::ClassFile;
use super::class_parser::parse;
use super::class_path_manager;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ClassLoader {
    parent_loader: Option<Arc<ClassLoader>>,
    cpm: Arc<class_path_manager::ClassPathManager>,
    loaded_class: Mutex<HashMap<String, Arc<Box<ClassFile>>>>,
}

impl ClassLoader {
    pub fn new(
        cpm: Arc<class_path_manager::ClassPathManager>,
        parent_loader: Option<Arc<ClassLoader>>,
    ) -> ClassLoader {
        ClassLoader {
            cpm,
            parent_loader,
            loaded_class: Mutex::new(HashMap::new()),
        }
    }

    pub fn load_class(&self, name: &str) -> Option<Arc<Box<ClassFile>>> {
        if let Some(parent_loader) = &self.parent_loader {
            let result = parent_loader.load_class(name);
            if result.is_some() {
                return result;
            }
        }

        let loaded_class = self.loaded_class.lock().unwrap();

        let result = loaded_class.get(name);
        if let Some(cls) = result {
            return Some(cls.clone());
        }

        match self.cpm.search_class(name) {
            Ok(class_path_manager::ClassPathResult(_, bytes)) => match parse(&bytes) {
                Ok(cf) => Some(Arc::new(Box::new(cf.1))),

                Err(e) => unreachable!("name={}, {}", name, e),
            },
            Err(_) => None,
        }
    }
}
