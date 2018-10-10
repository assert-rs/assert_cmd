#![feature(test)]

extern crate test;
extern crate escargot;

use std::process;

fn escargot_build() -> Result<process::Command, escargot::CargoError> {
    let cmd = escargot::CargoBuild::new()
        .bin("bin_fixture")
        .current_release()
        .current_target()
        .run()?
        .command();
    Ok(cmd)
}

// It is easier to benchmark this than to benchmark `main_binary` which would include the cost of
// running the binary.
#[bench]
fn escargot_rebuild(b: &mut test::Bencher) {
    let _ = escargot_build();
    b.iter(|| {let _ = escargot_build(); });
}
