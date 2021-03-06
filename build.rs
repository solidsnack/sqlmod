#[cfg(feature = "codegen")]
mod codegen {
    use std;
    extern crate cheddar;
    use self::cheddar::Cheddar;
    extern crate peg;

    pub fn header() {
        Cheddar::new().expect("Could not read manifest!")
            .insert_code("typedef struct sqlmod_queries_t sqlmod_queries_t;")
            .insert_code("\n")
            .insert_code("typedef struct sqlmod_query_t sqlmod_query_t;")
            .module("c").expect("Malformed module path!")
            .run_build("target/include/sqlmod.h");
    }

    pub fn peg() {
        let generated = concat!(env!("OUT_DIR"), "/parser.rs");
        let target = "src/peg.rs";
        peg::cargo_build("parser.rustpeg");
        std::fs::copy(generated, target)
                .expect(&format!("Could not copy `{}` to `{}`",
                                 generated, target));
    }

    pub fn activate() {
        peg();
        header();
    }
}


fn main() {
    #[cfg(feature = "codegen")]
    codegen::activate();
}
