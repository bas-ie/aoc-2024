use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub(crate) struct PuzzleInputAsset {
    #[allow(dead_code)]
    pub rows: Vec<Vec<i32>>,
}

#[derive(Default)]
pub(crate) struct PuzzleInputAssetLoader;

/// Possible errors that can be produced by [`PuzzleInputAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub(crate) enum PuzzleInputAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    /// A [string conversion](std::string::FromUtf8Error) error
    #[error("Could not parse utf8 from bytes: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),

    /// An [integer conversion](std::num::ParseIntError)
    #[error("Could not convert to integer: {0}")]
    IntegerConversionError(#[from] std::num::ParseIntError),
}

impl AssetLoader for PuzzleInputAssetLoader {
    type Asset = PuzzleInputAsset;
    type Settings = ();
    type Error = PuzzleInputAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let file_contents = String::from_utf8(bytes)?;
        let lines = file_contents.lines();

        let mut rows: Vec<Vec<i32>> = vec![];
        for line in lines {
            let cols: Vec<&str> = line.split(" ").collect();

            let mut row: Vec<i32> = vec![];
            for col in cols {
                if col.is_empty() {
                    continue;
                }
                let n: i32 = col.parse()?;
                row.push(n);
            }

            rows.push(row);
        }
        Ok(PuzzleInputAsset { rows })
    }

    fn extensions(&self) -> &[&str] {
        &["aoc"]
    }
}
