use std::ffi::OsStr;
use std::io::Write;
use libloading::Library;

pub struct Mod<'a> {
    // the library needs to be kept around for safety,
    // the other data is only safe to access as long as the library is alive
    #[allow(unused)]
    library: Library,
    pub name: &'a str,
    pub desc: &'a str,
    print: Option<unsafe extern fn()>
}

impl <'a> Mod<'a> {
    pub fn new<P: AsRef<OsStr>>(filename: P) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(filename)?;

            if let Ok(custom_func) = lib.get::<*mut fn() -> u32>(b"__CUSTOM_FUNC\0") {
                **custom_func = custom;
            }

            let name = **lib.get::<*const &str>(b"MOD_NAME\0")?;
            let desc = **lib.get::<*const &str>(b"MOD_DESC\0")?;
            let print = lib.get::<unsafe extern fn()>(b"print\0")
                .ok().map(|f| *f);
            Ok(Self { library: lib, name, desc, print })
        }
    }

    pub fn print(&self) {
        unsafe {
            match self.print {
                Some(f) => {
                    print!("\t\tprint function output: ");
                    std::io::stdout().flush().unwrap();
                    f()
                },
                None => {
                    println!("\t\tthere is no print function");
                }
            }
        }
    }
}

pub fn custom() -> u32 {
    51
}