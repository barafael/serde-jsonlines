#![cfg(feature = "async")]
use pin_project_lite::pin_project;
use serde::de::DeserializeOwned;
use std::io::Result;
use std::marker::Unpin;
use std::pin::Pin;
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

pin_project! {
    #[derive(Debug)]
    pub struct AsyncJsonLinesReader<R> {
        #[pin]
        inner: R,
    }
}

impl<R> AsyncJsonLinesReader<R> {
    /// Construct a new `AsyncJsonLinesReader` from a
    /// [`tokio::io::AsyncBufRead`] instance
    pub fn new(reader: R) -> Self {
        AsyncJsonLinesReader { inner: reader }
    }

    /// Consume the `AsyncJsonLinesReader` and return the underlying reader
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Get a reference to the underlying reader
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Get a mutable reference to the underlying reader
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Get a pinned mutable reference to the underlying reader
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut R> {
        self.project().inner
    }
}

impl<R: AsyncBufRead + Unpin> AsyncJsonLinesReader<R> {
    pub async fn read<T>(&mut self) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let mut s = String::new();
        let r = self.inner.read_line(&mut s).await?;
        if r == 0 {
            Ok(None)
        } else {
            Ok(Some(serde_json::from_str::<T>(&s)?))
        }
    }
}
