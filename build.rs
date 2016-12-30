extern crate cheddar;

use cheddar::Cheddar;


fn main() {
    Cheddar::new().expect("Could not read manifest!")
        .insert_code("typedef struct qselect_queries_t qselect_queries_t;")
        .insert_code("\n")
        .insert_code("typedef struct qselect_query_t qselect_query_t;")
        .module("c").expect("Malformed module path!")
        .run_build("target/include/qselect.h");
}