use modding_api::game_mod;

game_mod! {
    name = "Mod A"
    desc = "This is Mod A's description"
}

#[no_mangle]
pub extern "C" fn print() {
    println!("Hello from Mod A!");
}
