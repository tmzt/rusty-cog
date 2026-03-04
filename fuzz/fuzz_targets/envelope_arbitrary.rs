#![no_main]
use libfuzzer_sys::fuzz_target;

/// Structure-aware fuzzing: generate a JSON object with id + payload fields,
/// then attempt to dispatch.
fuzz_target!(|data: &[u8]| {
    // Try to parse as a full CogRequest
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(data) else {
        return;
    };

    let Ok(request) = serde_json::from_value::<cog_ndjson::CogRequest>(value) else {
        return;
    };

    // Verify the request can be dispatched (via default handler)
    smol::block_on(async {
        use cog_ndjson::handler::{DefaultHandler, RequestHandler};
        let handler = DefaultHandler;
        let _response = handler.handle(request).await;
    });
});
