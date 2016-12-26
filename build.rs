extern crate cheddar;

use cheddar::Cheddar;


fn main() {
    Cheddar::new().expect("Could not read manifest!")
        .module("c").expect("Malformed module path!")
        .run_build("target/include/qselect.h");
}