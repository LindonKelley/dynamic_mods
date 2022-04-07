use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Write;
use libloading::Library;

pub struct ModLoader<'a>{
    mods: Vec<(Mod<'a>, Vec<(&'a str, &'a [(&'a str, *mut *const ())])>)>,
    sources: HashMap<&'a str, HashMap<&'a str, *const ()>>,
}

impl <'a> ModLoader<'a> {
    pub fn new() -> Self {
        Self {
            mods: Vec::new(),
            sources: HashMap::new()
        }
    }

    pub fn insert<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<(), ModInsertionError> {
        unsafe {
            let mo = Mod::new(filename)?;

            let mod_name = mo.name.clone();
            if self.sources.remove(&mod_name).is_some() {
                return Err(ModInsertionError::ModAlreadyExists(mo.name().clone().into()));
            }
            let sources = self.sources.entry(mod_name).or_insert(HashMap::new());
            if let Ok(exports) = mo.library.get::<*const &[(*const (), &str)]>(b"__EXPORT\0") {
                println!("{} has exports", mo.name);
                for (source, alias_name) in **exports {
                    println!("\texporting address '{:?}' as '{}'", source, alias_name);
                    sources.insert(*alias_name, *source);
                }
            }

            let mut mod_imports = Vec::new();
            if let Ok(imports) = mo.library.get::<*const &[(&str, &[(&str, *mut *const ())])]>(b"__IMPORT\0") {
                println!("{} has imports", mod_name);
                for (source_mod, targets) in **imports {
                    println!("\timporting {} variables from '{}'", targets.len(), source_mod);
                    mod_imports.push((*source_mod, *targets));
                }
            }

            self.mods.push((mo, mod_imports));
            Ok(())
        }
    }

    /// resolves imports and exports from the contained mods, returning these resolved mods, or
    /// the error that occurred
    pub fn resolve(self) -> Result<Vec<Mod<'a>>, ModResolverError> {
        let mut mods = Vec::with_capacity(self.mods.len());
        for (target_mod, mod_imports) in self.mods {
            for (source_mod_name, imports) in mod_imports {
                let source_mod = self.sources.get(source_mod_name)
                    .ok_or_else(|| ModResolverError::source_mod_missing(source_mod_name, target_mod.name()))?;
                for (source_variable_name, target) in imports {
                    let source = source_mod.get(source_variable_name)
                        .ok_or_else(|| ModResolverError::source_variable_missing(source_mod_name, source_variable_name, target_mod.name))?;
                    unsafe {
                        **target = *source;
                    }
                }
            }
            mods.push(target_mod);
        }
        Ok(mods)
    }
}

#[derive(Debug)]
pub enum ModInsertionError {
    ModLoaderError(ModLoaderError),
    ModAlreadyExists(String)
}

impl From<ModLoaderError> for ModInsertionError {
    fn from(e: ModLoaderError) -> Self {
        Self::ModLoaderError(e)
    }
}

#[derive(Debug)]
pub enum ModResolverError {
    SourceModMissing {
        source: String,
        target: String
    },
    SourceVariableMissing {
        source_mod: String,
        source_variable: String,
        target_mod: String
    }
}

impl ModResolverError {
    fn source_mod_missing(source: &str, target: &str) -> Self {
        Self::SourceModMissing {
            source: source.into(),
            target: target.into()
        }
    }

    fn source_variable_missing(source_mod: &str, source_variable: &str, target_mod: &str) -> Self {
        Self::SourceVariableMissing {
            source_mod: source_mod.into(),
            source_variable: source_variable.into(),
            target_mod: target_mod.into()
        }
    }
}

pub struct Mod<'a> {
    // the library needs to be kept around for safety,
    // the other data is only safe to access as long as the library is alive
    #[allow(unused)]
    library: Library,
    name: &'a str,
    desc: &'a str,
    print: Option<unsafe extern fn()>
}

impl <'a> Mod<'a> {
    pub fn new<P: AsRef<OsStr>>(filename: P) -> Result<Self, ModLoaderError> {
        unsafe {
            let lib = Library::new(filename)
                .map_err(|e| ModLoaderError::LoadingError(e))?;

            if let Ok(custom_func) = lib.get::<*mut fn() -> u32>(b"__CUSTOM_FUNC\0") {
                **custom_func = custom;
            }

            let name = **lib.get::<*const &str>(b"__MOD_NAME\0")
                .map_err(|e| ModLoaderError::NameError(e))?;
            let desc = **lib.get::<*const &str>(b"__MOD_DESC\0")
                .map_err(|e| ModLoaderError::DescError(e))?;

            let print = lib.get::<unsafe extern fn()>(b"print\0")
                .ok().map(|f| *f);

            Ok(Self { library: lib, name, desc, print })
        }
    }

    /// gets the name of this mod
    ///
    /// **note**: *this function exists here, rather than setting the visibility of name to pub in
    /// order to prevent mutation of the name, as it's really a reference to the actual static name
    /// in the mod's loaded binary, and mutating it could be unsafe*
    #[inline(always)]
    pub fn name(&self) -> &str {
        self.name
    }

    /// gets the description of this mod
    ///
    /// **note**: *this function exists here, rather than setting the visibility of desc to pub in
    /// order to prevent mutation of the desc, as it's really a reference to the actual static name
    /// in the mod's loaded binary, and mutating it could be unsafe*
    #[inline(always)]
    pub fn desc(&self) -> &str {
        self.desc
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

#[derive(Debug)]
pub enum ModLoaderError {
    LoadingError(libloading::Error),
    NameError(libloading::Error),
    DescError(libloading::Error)
}

pub fn custom() -> u32 {
    51
}