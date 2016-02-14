#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate clap;

extern crate libimagcounter;
extern crate libimagrt;
extern crate libimagutil;

use std::process::exit;

use libimagrt::runtime::Runtime;

mod create;
mod delete;
mod ui;

use ui::build_ui;
use create::create;
use delete::delete;

fn main() {
    let name = "imag-counter";
    let version = &version!()[..];
    let about = "Counter tool to count things";
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

    rt.cli()
        .subcommand_name()
        .map_or_else(|| {
                // No subcommand, we only have args.
                unimplemented!()
            },
            |name| {
                debug!("Call: {}", name);
                match name {
                    "create" => create(&rt),
                    "delete" => delete(&rt),
                    _ => {
                        debug!("Unknown command"); // More error handling
                    },
                };
            })



}
