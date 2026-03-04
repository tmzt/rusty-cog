//! I/O adapter bridging `futures_io` (smol) to `hyper::rt` traits.

use std::pin::Pin;
use std::task::{Context, Poll};

/// Wraps a `futures_io::AsyncRead + AsyncWrite` stream so it implements
/// `hyper::rt::Read + hyper::rt::Write`.
pub(crate) struct SmolIo<T>(pub T);

impl<T> hyper::rt::Read for SmolIo<T>
where
    T: futures_lite::AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: hyper::rt::ReadBufCursor<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let this = self.get_mut();
        // SAFETY: We only advance `buf` by the number of bytes actually written
        // by `poll_read`. `futures_io::AsyncRead::poll_read` writes into the
        // slice before returning the count, so reading uninitialised memory is
        // avoided in practice.
        let slice = unsafe {
            &mut *(buf.as_mut() as *mut [std::mem::MaybeUninit<u8>] as *mut [u8])
        };
        match Pin::new(&mut this.0).poll_read(cx, slice) {
            Poll::Ready(Ok(n)) => {
                unsafe { buf.advance(n) };
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T> hyper::rt::Write for SmolIo<T>
where
    T: futures_lite::AsyncWrite + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.get_mut().0).poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.get_mut().0).poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.get_mut().0).poll_close(cx)
    }
}
