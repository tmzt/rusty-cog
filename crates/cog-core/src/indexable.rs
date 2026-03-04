use crate::error::Result;

/// Trait for services that support offline content indexing.
///
/// The indexing engine (`rusty-genius`) is a separate component. `cog-core`
/// defines this trait; individual services implement it. The engine calls
/// `fetch_indexable` to pull documents, persists them, and answers queries
/// via the `Index` protocol messages.
///
/// # Type Parameters
///
/// - `Document`: The structured document type yielded by this service.
///   Must be serializable so the indexing engine can persist it.
///
/// # Cursor
///
/// Each service uses a cursor string to track incremental progress:
///
/// | Service | Cursor type |
/// |---------|-------------|
/// | Gmail   | `historyId` |
/// | Drive   | `changeToken` |
/// | Docs    | `modifiedTime` |
/// | Keep    | `updateTime` |
pub trait Indexable {
    /// The document type produced by this service for indexing.
    type Document: serde::Serialize;

    /// Fetch documents modified since the given cursor.
    ///
    /// Returns a batch of documents and an optional new cursor. When the
    /// cursor is `None`, the caller should start from the beginning.
    /// When the returned cursor is `None`, there are no more documents.
    ///
    /// # Arguments
    ///
    /// - `since`: Cursor from a previous call, or `None` for initial sync.
    /// - `limit`: Maximum number of documents to return in this batch.
    fn fetch_indexable(
        &self,
        since: Option<&str>,
        limit: usize,
    ) -> impl std::future::Future<Output = Result<(Vec<Self::Document>, Option<String>)>> + Send;

    /// The namespace identifier for this service's index data.
    ///
    /// Used as a directory name under `$COG_HOME/index/` and as a
    /// discriminator in index queries.
    fn index_namespace(&self) -> &'static str;
}
