#![no_main]
use libfuzzer_sys::fuzz_target;

/// Fuzz a multi-message NDJSON stream by splitting on newlines.
fuzz_target!(|data: &[u8]| {
    for line in data.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }
        let mut with_newline = line.to_vec();
        with_newline.push(b'\n');
        let _ = cog_ndjson::wire::decode::<cog_ndjson::CogRequest>(&with_newline);
    }
});
