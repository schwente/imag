extern crate clap;
#[macro_use] extern crate log;
extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;

use libimagrt::runtime::Runtime;
use std::process::exit;

mod ui;

use ui::build_ui;

fn main() {
    let name = "imag-view";
    let version = &version!()[..];
    let about = "View entries (readonly)";
    let ui = build_ui(Runtime::get_default_cli_builder(name, version, about));
    let rt = {
        let rt = Runtime::new(ui);
        if rt.is_ok() {
            rt.unwrap()
        } else {
            println!("Could not set up Runtime");
            println!("{:?}", rt.err().unwrap());
            exit(1);
        }
    };

    rt.init_logger();

    debug!("Hello. Logging was just enabled");
    debug!("I already set up the Runtime object and build the commandline interface parser.");
    debug!("Lets get rollin' ...");

    info!("No implementation yet");
}
