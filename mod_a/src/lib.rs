// pub isn't actually necessary on any of these, it just feels
// appropriate to add them in

#[no_mangle]
static mut __CUSTOM_FUNC: fn() -> u32 = custom;

pub fn custom() -> u32 {
    unsafe { __CUSTOM_FUNC() }
}

#[no_mangle]
pub static MOD_NAME: &str = "Mod A";
#[no_mangle]
pub static MOD_DESC: &str = "This is Mod A's description";

#[no_mangle]
pub extern "C" fn print() {
    println!("Hello from Mod A! {}", custom());
}
