extern crate libnewsboat;
extern crate tempfile;

use libnewsboat::logger::get_instance;
use self::tempfile::TempDir;

#[test]
fn t_get_instance_returns_valid_logger() {
    let tmp = TempDir::new().unwrap();
    let logfile = {
        let mut logfile = tmp.path().to_owned();
        logfile.push("example.log");
        logfile
    };

    assert!(!logfile.exists());

    get_instance().set_logfile(logfile.to_str().unwrap());

    assert!(logfile.exists());
}
