use crate::error::{Error, Result};

/// Maximum number of items allowed in a bulk operation without the
/// `destructive-bulk-trash` feature gate.
pub const BULK_TRASH_THRESHOLD: usize = 50;

/// Check whether a bulk trash operation is allowed.
///
/// When the `destructive-bulk-trash` feature is enabled, this always succeeds.
/// When disabled, it returns `Error::BulkTrashDenied` if `count > 50`.
pub fn check_bulk_trash(count: usize) -> Result<()> {
    if cfg!(feature = "destructive-bulk-trash") {
        return Ok(());
    }

    if count > BULK_TRASH_THRESHOLD {
        return Err(Error::BulkTrashDenied { count });
    }

    Ok(())
}

/// Marker trait for types that represent permanently destructive operations.
///
/// This trait exists only when the `destructive-permanent` feature is enabled.
/// It serves as compile-time documentation that a type or method performs
/// irreversible deletion.
#[cfg(feature = "destructive-permanent")]
pub trait PermanentlyDestructive {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bulk_trash_within_threshold() {
        assert!(check_bulk_trash(50).is_ok());
        assert!(check_bulk_trash(1).is_ok());
        assert!(check_bulk_trash(0).is_ok());
    }

    #[test]
    fn bulk_trash_exceeds_threshold() {
        let result = check_bulk_trash(51);
        if cfg!(feature = "destructive-bulk-trash") {
            assert!(result.is_ok());
        } else {
            assert!(result.is_err());
            match result.unwrap_err() {
                Error::BulkTrashDenied { count } => assert_eq!(count, 51),
                e => panic!("unexpected error: {e}"),
            }
        }
    }
}
