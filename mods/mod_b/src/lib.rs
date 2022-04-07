use modding_api::game_mod;

game_mod! {
    name = "Mod B",
    desc = "This is Mod B's description"
}

#[no_mangle]
pub static mut __IMPORT: &[(&str, &[(&str, *mut *const ())])] = &[
    ("Mod C", &[
        // import "special" as __SPECIAL_FUNC from 'Mod C'
        // gonna have to ensure import statics are mutable, so they don't get placed in immutable memory
        ("special\0", unsafe { &__SPECIAL_FUNC as *const _ as *mut () as *mut *const () })
    ])
];

static mut __SPECIAL_FUNC: fn(u32, u32) -> u32 = special;

pub fn special(a: u32, b: u32) -> u32 {
    unsafe { __SPECIAL_FUNC(a, b) }
}

#[no_mangle]
pub extern "C" fn print() {
    println!("special call from Mod B! {}==15", special(5, 3));
}
