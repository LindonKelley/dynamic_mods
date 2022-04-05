// pub isn't actually necessary on any of these, it just feels
// appropriate to add them in

#[no_mangle]
pub static MOD_NAME: &str = "Mod C";
#[no_mangle]
pub static MOD_DESC: &str = "This is Mod C's description";

#[no_mangle]
pub extern "C" fn print() {
    println!("Hello from Mod C!");
}
