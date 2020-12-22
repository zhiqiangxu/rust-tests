use std::fs::{read, File};
use std::io::{self, Read};
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use tracing::info;
use zip::ZipArchive;

use super::util;

enum ClassPathEntry {
    Dir(String),
    Jar(ZipRef, String),
}

pub struct ClassPathResult(pub String, pub Vec<u8>);

type ZipRef = Arc<Mutex<Box<ZipArchive<File>>>>;

pub struct ClassPathManager {
    class_path: RwLock<Vec<ClassPathEntry>>,
}

impl ClassPathManager {
    pub fn new() -> Self {
        Self {
            class_path: RwLock::new(vec![]),
        }
    }

    pub fn add_class_path(&mut self, path: &str) -> Result<(), io::Error> {
        let p = Path::new(path);
        if p.is_dir() {
            self.class_path
                .write()
                .unwrap()
                .push(ClassPathEntry::Dir(path.to_string()));
        } else {
            let f = File::open(p)?;
            let z = ZipArchive::new(f)?;
            let handle = Arc::new(Mutex::new(Box::new(z)));
            self.class_path
                .write()
                .unwrap()
                .push(ClassPathEntry::Jar(handle, path.to_string()));
        }

        Ok(())
    }

    pub fn add_class_paths(&mut self, path: &str) -> Result<(), io::Error> {
        path.split(util::PATH_SEP)
            .try_for_each(|p| -> Result<(), io::Error> { self.add_class_path(p) })
    }

    pub fn search_class(&self, name: &str) -> Result<ClassPathResult, io::Error> {
        let name = name.replace(".", util::FILE_SEP);

        info!("search_class: {}", name);

        for it in self.class_path.read().unwrap().iter() {
            match it {
                ClassPathEntry::Dir(path) => {
                    let mut p = String::from(path);
                    p.push_str(util::FILE_SEP);
                    p.push_str(&name);
                    p.push_str(".class");
                    if let Ok(data) = read(&p) {
                        return Ok(ClassPathResult(p, data));
                    }
                }
                ClassPathEntry::Jar(handle, path) => {
                    let mut p = String::from(&name);
                    p.push_str(".class");

                    let mut handle = handle.lock().unwrap();
                    let zf = handle.by_name(&p);

                    if let Ok(mut zf) = zf {
                        let mut v = Vec::with_capacity(zf.size() as usize);
                        let r = zf.read_to_end(&mut v);

                        debug_assert!(r.is_ok());

                        return Ok(ClassPathResult(path.clone(), v));
                    }
                }
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Search class failed: {}", name),
        ))
    }

    pub fn size(&self) -> usize {
        self.class_path.read().unwrap().len()
    }
}
