extern crate pkg_config;

fn main() {
    pkg_config::find_library("pocketsphinx").unwrap();
    pkg_config::find_library("sphinxbase").unwrap();
}
