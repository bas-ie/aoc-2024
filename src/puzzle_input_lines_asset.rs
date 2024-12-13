use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub(crate) struct PuzzleInputLinesAsset {
    #[allow(dead_code)]
    pub rows: Vec<String>,
}

#[derive(Default)]
pub(crate) struct PuzzleInputLinesAssetLoader;

/// Possible errors that can be produced by [`PuzzleInputLinesAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub(crate) enum PuzzleInputLinesAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    /// A [string conversion](std::string::FromUtf8Error) error
    #[error("Could not parse utf8 from bytes: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

// We need a raw asset loader that only grabs strings, and very little other processing is done.
impl AssetLoader for PuzzleInputLinesAssetLoader {
    type Asset = PuzzleInputLinesAsset;
    type Settings = ();
    type Error = PuzzleInputLinesAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let file_contents = String::from_utf8(bytes)?;
        let rows = file_contents.lines().map(|l| l.to_string()).collect();

        Ok(PuzzleInputLinesAsset { rows })
    }

    fn extensions(&self) -> &[&str] {
        &["aoc"]
    }
}
