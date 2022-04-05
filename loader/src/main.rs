mod loader;

use std::env;
use crate::loader::Mod;

fn main() {
    let mut exe = env::current_exe()
        .expect("current directory may not exist,\
        or this program was not run with the privileges to access it");
    exe.pop();
    let mods_dir = exe
        .join("mods")
        .read_dir()
        .expect("could not read 'mods' directory");
    let mut mods = Vec::new();
    for result in mods_dir {
        let mod_path = match result {
            Ok(entry) => entry.path(),
            Err(_) => {
                eprintln!("unable to read individual mod path, skipping");
                continue
            }
        };
        match Mod::new(mod_path) {
            // I know I can use r#mod here but r# just feels wrong to me
            Ok(mo) => mods.push(mo),
            Err(e) => eprintln!("unable to load mod, error: {}", e),
        };
    }

    println!("{} mods found: ", mods.len());
    for mo in mods {
        println!("\t{}", mo.name);
        println!("\t\t{}", mo.desc);
        mo.print();
    }
}

#[no_mangle]
extern "C" fn custom() -> u32 {
    31
}
