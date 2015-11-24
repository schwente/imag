extern crate clap;
use clap::{App, ArgMatches};

pub struct ModuleConfig {
    pub load : bool,
}

impl ModuleConfig {
    pub fn new() -> ModuleConfig {
        ModuleConfig {
            load: false,
        }
    }
}

pub struct CliConfig<'a> {
    pub module_configs  : Vec<ModuleConfig>,
    pub cli_matches     : ArgMatches<'a, 'a>,
}

impl<'a> CliConfig<'a> {
    pub fn new(app : clap::App<'a, 'a, 'a, 'a, 'a, 'a>) -> CliConfig<'a> {
        CliConfig {
            module_configs: vec![],
            cli_matches: app.get_matches(),
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.cli_matches.is_present("verbose") || self.is_debugging()
    }

    pub fn is_debugging(&self) -> bool {
        self.cli_matches.is_present("debug")
    }

    pub fn get_rtp(&self) -> Option<String> {
        self.cli_matches.value_of("rtp").and_then(|s| Some(String::from(s)))
    }
}

