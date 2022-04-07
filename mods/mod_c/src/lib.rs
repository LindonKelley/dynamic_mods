use modding_api::game_mod;

game_mod! {
    name = "Mod C";
    desc = "This is Mod C's description";
}

#[no_mangle]
pub static mut __EXPORT: &[(*const (), &str)] = &[
    // export pointer to special as "special"
    (special as *const (), "special\0")
];

pub fn special(a: u32, b: u32) -> u32 {
    a * b
}
