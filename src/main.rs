extern crate clap;
use clap::{Arg, App};

static PROGRAM_NAME : &str = "bckpcln";
static PROGRAM_VERSION : &str = "1.0";

fn main() {
    let app = App::new(PROGRAM_NAME)
        .version(PROGRAM_VERSION)
        .author("rodolphe saugier <rodolphe.saugier@gmail.com>")
        .about("BaCKuP CLeaNer. Removes old backups from a directory containing many.")
        .arg(Arg::with_name("directory")
             .short("d")
             .long("directory")
             .takes_value(true));
    let args = app.get_matches();
}
