extern crate dirs;
use std::path::PathBuf;

#[allow(dead_code)]
fn get_db_file() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push("wk.db");
    return path;
    
}

#[cfg (test)] 
mod tests {
    use std::path::PathBuf;
    use super::get_db_file;

    #[test]
    fn can_get_homedir() {
        assert_eq!(dirs::home_dir(), Some(PathBuf::from("/home/johnlockwood")));
    }

    #[test]
    fn can_get_db_file() {
        let file = get_db_file();
        assert_eq!(file, PathBuf::from("/home/johnlockwood/wk.db"));
    }
}