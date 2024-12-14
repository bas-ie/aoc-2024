#![allow(dead_code)]
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub struct PuzzleInputStringAsset(pub String);

#[derive(Default)]
pub struct PuzzleInputStringAssetLoader;

/// Possible errors that can be produced by [`PuzzleInputStringAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PuzzleInputStringAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    /// A [string conversion](std::string::FromUtf8Error) error
    #[error("Could not parse utf8 from bytes: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

// We need a raw asset loader that only grabs strings, and very little other processing is done.
impl AssetLoader for PuzzleInputStringAssetLoader {
    type Asset = PuzzleInputStringAsset;
    type Settings = ();
    type Error = PuzzleInputStringAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        Ok(PuzzleInputStringAsset(String::from_utf8(bytes)?))
    }

    fn extensions(&self) -> &[&str] {
        &["aoc"]
    }
}
