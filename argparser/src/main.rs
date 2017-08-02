// #![cfg_attr(feature="clippy", feature(plugin))]
#![feature(plugin)]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate clap;

use clap::{App, Arg};

#[cfg(foo)]
struct Foo;

fn main() {
    let matches = App::new("DemoApp")
        .long_version("1.0")
        .author("baihe")
        .about("demo app to indicate use of clap")
        .arg(Arg::with_name("arg1")
            .short("A")
            .long("arg1")
            .value_name("ARG1")
            .required(true)
            .help("the first argument")
            .takes_value(true))
        .get_matches();

    let val1 = match matches.value_of("arg1") {
        Some(v) => v,
        None => "default_value"
    };

    println!("{}", val1);
}
