use cog_core::destructive;

#[test]
fn bulk_trash_within_threshold_allowed() {
    assert!(destructive::check_bulk_trash(50).is_ok());
    assert!(destructive::check_bulk_trash(1).is_ok());
    assert!(destructive::check_bulk_trash(0).is_ok());
}

#[test]
fn bulk_trash_at_boundary() {
    // Exactly 50 should be allowed
    assert!(destructive::check_bulk_trash(50).is_ok());

    // 51 depends on feature flag
    let result = destructive::check_bulk_trash(51);
    if cfg!(feature = "destructive-bulk-trash") {
        assert!(result.is_ok(), "51 items should be allowed with feature enabled");
    } else {
        assert!(result.is_err(), "51 items should be denied without feature");
        match result.unwrap_err() {
            cog_core::Error::BulkTrashDenied { count } => {
                assert_eq!(count, 51);
            }
            e => panic!("expected BulkTrashDenied, got: {e}"),
        }
    }
}

#[test]
fn bulk_trash_large_count() {
    let result = destructive::check_bulk_trash(10000);
    if cfg!(feature = "destructive-bulk-trash") {
        assert!(result.is_ok());
    } else {
        assert!(result.is_err());
    }
}

#[test]
fn error_exit_code_for_permanent_delete_denied() {
    let err = cog_core::Error::PermanentDeleteDenied;
    assert_eq!(err.exit_code(), 1);
}

#[test]
fn error_exit_code_for_bulk_trash_denied() {
    let err = cog_core::Error::BulkTrashDenied { count: 100 };
    assert_eq!(err.exit_code(), 1);
}

// Verify that destructive methods exist or don't based on feature flags
#[cfg(feature = "destructive-permanent")]
#[test]
fn destructive_permanent_methods_available() {
    // With the feature enabled, destructive methods should compile
    // This test just verifies compilation
    use cog_core::services::gmail::GmailService;
    use cog_core::http::HttpClient;

    let client = HttpClient::new().unwrap();
    let service = GmailService::new(client, "test_token".to_string());

    // These methods only exist with destructive-permanent feature
    let _ = smol::block_on(service.delete("msg123"));
    let _ = smol::block_on(service.labels_delete("label123"));
    let _ = smol::block_on(service.filters_delete("filter123"));
    let _ = smol::block_on(service.batch_delete(&["msg1".to_string(), "msg2".to_string()]));
}
