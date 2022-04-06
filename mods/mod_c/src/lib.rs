// pub isn't actually necessary on any of these, it just feels
// appropriate to add them in

#[no_mangle]
pub static MOD_NAME: &str = "Mod C";
#[no_mangle]
pub static MOD_DESC: &str = "This is Mod C's description";
#[no_mangle]
pub static mut __EXPORT: &[(*const (), &str)] = &[
    // export pointer to special as "special"
    (special as *const (), "special\0")
];

pub fn special(a: u32, b: u32) -> u32 {
    a * b
}
