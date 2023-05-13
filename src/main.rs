extern crate crypto;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;


mod dirspec;

use std::env;
use dirspec::DirSpec;

fn main() {
    env_logger::init().unwrap();

    // let path = r#"G:\dev\ha\haterm\src"#;
    //let dp = r#"G:\tmp"#;
    let dp = ".\\target";
    let path = {
        let mut args = env::args();
        args.next();
        if let Some(ap) = args.next() {
            ap
        } else {
            dp.to_owned()
        }
    };

    trace!("Root: {}", path);
    
    let d1: DirSpec = DirSpec::new(path).unwrap();
    // println!("{:?}", d1);
    // println!("---");
    println!("{}", d1.hash());
}
