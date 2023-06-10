use uuid::Uuid;

use crate::string_utils;

pub fn print_rand_uuid() {
    let uuid = Uuid::new_v4();
    println!("{}", uuid);
}

pub fn print_win_to_cyg_path(win_path: &str) {
    println!("{}", string_utils::win_to_cyg_path(win_path));
}
