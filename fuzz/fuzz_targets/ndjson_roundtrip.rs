#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(data) else {
        return;
    };

    let Ok(request) = serde_json::from_value::<cog_ndjson::CogRequest>(value) else {
        return;
    };

    let encoded = cog_ndjson::wire::encode(&request).unwrap();
    let decoded: cog_ndjson::CogRequest = cog_ndjson::wire::decode(&encoded).unwrap();
    assert_eq!(request.id, decoded.id);
});
