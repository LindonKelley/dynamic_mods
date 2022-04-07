mod loader;

use crate::loader::ModLoader;
use std::env;

fn main() {
    let mut exe = env::current_exe().expect(
        "current directory may not exist,\
        or this program was not run with the privileges to access it",
    );
    exe.pop();
    let mods_dir = exe
        .join("mods")
        .read_dir()
        .expect("could not read 'mods' directory");
    let mut mod_loader = ModLoader::new();
    for result in mods_dir {
        let mod_path = match result {
            Ok(entry) => entry.path(),
            Err(_) => {
                eprintln!("unable to read individual mod path, skipping");
                continue;
            }
        };
        if let Err(e) = mod_loader.insert(mod_path) {
            eprintln!("unable to load mod, error: {:?}", e);
        }
    }

    match mod_loader.resolve() {
        Ok(mods) => {
            println!("{} mods found: ", mods.len());
            for mo in &mods {
                println!("\t{}", mo.name());
                println!("\t\t{}", mo.desc());
                mo.print();
            }
        }
        Err(e) => {
            eprintln!("unable to resolve mods {:?}", e);
        }
    }
}

#[no_mangle]
extern "C" fn custom() -> u32 {
    31
}
