#![no_main]
use libfuzzer_sys::fuzz_target;

/// Fuzz the handshake parsing.
fuzz_target!(|data: &[u8]| {
    // Try to parse as a handshake request
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(req) = serde_json::from_str::<cog_ndjson::handshake::HandshakeRequest>(s) {
            let _ = req.validate();
        }
    }

    // Also try as handshake response
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = serde_json::from_str::<cog_ndjson::handshake::HandshakeResponse>(s);
    }
});
