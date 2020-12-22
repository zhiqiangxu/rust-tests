use base58::FromBase58;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use syn::Item;
#[macro_use]
extern crate quote;

fn main() -> Result<(), Box<Error>> {
    let b58 = "AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV".from_base58().unwrap();
    let mut content = String::new();
    let clone_content = content.clone();
    println!("test:{} b58:{:?}", &content == &clone_content, b58);

    let mut file = File::open("/Users/xuzhiqiang/.gvm/pkgsets/go1.10/global/src/github.com/zhiqiangxu/codecombinator/core/src/operator/sql.rs")?;

    file.read_to_string(&mut content)?;

    let ast = syn::parse_file(&content)?;

    let mut operators = vec![];
    let mut configs = vec![];
    let mut monads = HashMap::<_, _>::new();
    let mut impls = HashMap::<_, _>::new();

    // let mut applies
    for i in &ast.items {
        match i {
            Item::Impl(iimpl) => {
                impls.insert(&iimpl.self_ty, iimpl);
                match iimpl {
                    syn::ItemImpl {
                        self_ty, trait_, ..
                    } => match trait_ {
                        Some((_, syn::Path { segments, .. }, _)) => {
                            let last_segment = segments.last().unwrap();
                            if last_segment.ident == "Monad" {
                                let entry = monads.entry(self_ty).or_insert(vec![]);
                                entry.push(&last_segment.arguments);
                            } else if last_segment.ident == "Operator"
                                || last_segment.ident == "Source"
                            {
                                operators.push(self_ty);
                            }
                        }
                        _ => {}
                    },
                }

                // println!("self:{}", quote!(#self_ty),);
            }
            Item::Struct(s) => {
                if s.ident != "Config" {
                    continue;
                }

                configs.push(s);
            }
            _ => {}
        }
    }

    println!(
        "operators: {:?} \nconfigs: {:?} \nmonads: {:?}",
        operators, configs, monads
    );

    for op in operators {}
    Ok(())
}
