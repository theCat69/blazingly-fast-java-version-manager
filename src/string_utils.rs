#[cfg(target_family = "windows")]
pub fn win_to_cyg_path(win_path: &str) -> String {
    let win_path_splitted = win_path.split(";").map(|st| st.to_string());
    let mut result_vec: Vec<String> = Vec::new();
    for mut path_member in win_path_splitted.into_iter() {
        let s = path_member.get_mut(0..1);
        let t = s.map(|s| {
            let mut s = s.to_string();
            s.make_ascii_lowercase();
            let s = "/".to_string() + &s;
            s
        });

        match t {
            Some(windisk) => {
                path_member.replace_range(0..2, &windisk);
                let path_member = path_member.replace("\\", "/");
                result_vec.push(path_member);
            }
            None => panic!("Cannot format windows disk reference to cyg path"),
        }
    }
    result_vec.join(":")
}
