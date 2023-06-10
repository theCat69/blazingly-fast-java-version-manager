// I was thinking of using a memory file written in binary instead of json
// And compare a sha256 hash from both config to know if load from json was
// necessary because the user uptaded it but i saw only a 10% gain in
// performance by using this approch in a benchmark so i postponed it to later if config
// get really big and i need to hide some values i need to persist

// The benchmark :
// use std::time::Instant;
// use std::{
//     collections::HashMap,
//     error::Error,
//     fs,
//     path::{Path, PathBuf},
// };
//
// use serde::{Deserialize, Serialize};
//
// type Result<T> = std::result::Result<T, Box<dyn Error>>;
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct JavaVersion {
//     pub home_folder: String,
//     pub installed: bool,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Config {
//     pub user_current_jdk: PathBuf,
//     pub java_versions: HashMap<String, JavaVersion>,
// }
//
// fn main() {
//     // let config = load_config_from_file().expect("To be able to read config");
//     // let _ = dump_binaries(&config);
//     let sha_saved = "c11478bdcfcf259ab8384f7c4e92f69616f5ae6a609886d6bd21dc246dadf4d3";
//     let before = Instant::now();
//     for _ in 0..9 {
//         let _ = read_hash_and_compare(sha_saved);
//     }
//     println!("Elapsed time: {:.2?}", before.elapsed());
//
//     let before_100 = Instant::now();
//     for _ in 0..99 {
//         let _ = read_hash_and_compare(sha_saved);
//     }
//     println!("Elapsed time: {:.2?}", before_100.elapsed());
//
//     let before_1000 = Instant::now();
//     for _ in 0..999 {
//         let _ = read_hash_and_compare(sha_saved);
//     }
//     println!("Elapsed time: {:.2?}", before_1000.elapsed());
//
//     let before_10000 = Instant::now();
//     for _ in 0..9999 {
//         let _ = read_hash_and_compare(sha_saved);
//     }
//     println!("Elapsed time: {:.2?}", before_10000.elapsed());
//
//     let before_json = Instant::now();
//     for _ in 0..9 {
//         let _ = load_config_from_file();
//     }
//     println!("Elapsed time: {:.2?}", before_json.elapsed());
//
//     let before_json_100 = Instant::now();
//     for _ in 0..99 {
//         let _ = load_config_from_file();
//     }
//     println!("Elapsed time: {:.2?}", before_json_100.elapsed());
//
//     let before_json_1000 = Instant::now();
//     for _ in 0..999 {
//         let _ = load_config_from_file();
//     }
//     println!("Elapsed time: {:.2?}", before_json_1000.elapsed());
//
//     let before_json_10000 = Instant::now();
//     for _ in 0..9999 {
//         let _ = load_config_from_file();
//     }
//     println!("Elapsed time: {:.2?}", before_json_10000.elapsed());
// }
//
// fn load_config_from_file() -> Result<Config> {
//     let file = fs::read("./bf-j-vm.json")?;
//     Ok(serde_json::from_slice::<Config>(&file)?)
// }
//
// fn dump_binaries(config: &Config) -> Result<()> {
//     fs::write(Path::new("./bf-j-vm"), bincode::serialize(config)?)?;
//     Ok(())
// }
//
// fn load_config_from_binaries() -> Result<Config> {
//     let file = fs::read("./bf-j-vm")?;
//     Ok(bincode::deserialize(&file)?)
// }
//
// fn read_hash_and_compare(saved_hash: &str) -> bool {
//     let file_path = Path::new("./bf-j-vm.json");
//     let sha = sha256::try_digest(file_path).expect("to be able to digest json");
//     sha.as_str() == saved_hash
// }
//

pub struct Memory {
    config: Config,
}
