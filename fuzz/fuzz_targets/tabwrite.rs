#![no_main]
use std::io::Write;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let sink = std::io::sink();
    let mut tw = tabwriter::TabWriter::new(sink);
    let _ = tw.write_all(data);
});
