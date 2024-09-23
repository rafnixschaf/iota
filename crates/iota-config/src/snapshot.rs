// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    str::FromStr,
};

use anyhow::bail;
use flate2::bufread::GzDecoder;
use iota_sdk::Url;
use serde::{Deserialize, Serialize};

pub const IOTA_OBJECT_SNAPSHOT_URL: &str = "https://stardust-objects.s3.eu-central-1.amazonaws.com/iota/alphanet/latest/stardust_object_snapshot.bin.gz";
pub const SHIMMER_OBJECT_SNAPSHOT_URL: &str = "https://stardust-objects.s3.eu-central-1.amazonaws.com/shimmer/alphanet/latest/stardust_object_snapshot.bin.gz";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SnapshotSource {
    /// Local uncompressed file.
    Local(PathBuf),
    /// Remote file (S3) with gzip compressed file
    S3(SnapshotUrl),
}

impl SnapshotSource {
    /// Convert to a reader.
    pub fn to_reader(&self) -> anyhow::Result<Box<dyn Read>> {
        Ok(match self {
            SnapshotSource::Local(path) => Box::new(BufReader::new(File::open(path)?)),
            SnapshotSource::S3(snapshot_url) => Box::new(snapshot_url.to_reader()?),
        })
    }
}

impl From<SnapshotUrl> for SnapshotSource {
    fn from(value: SnapshotUrl) -> Self {
        Self::S3(value)
    }
}

/// The URLs to download Iota or Shimmer object snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum SnapshotUrl {
    Iota,
    Shimmer,
    /// Custom migration snapshot for testing purposes.
    Test(Url),
}

impl std::fmt::Display for SnapshotUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnapshotUrl::Iota => "iota".fmt(f),
            SnapshotUrl::Shimmer => "smr".fmt(f),
            SnapshotUrl::Test(url) => url.as_str().fmt(f),
        }
    }
}

impl FromStr for SnapshotUrl {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(url) = reqwest::Url::from_str(s) {
            return Ok(Self::Test(url));
        }
        Ok(match s.to_lowercase().as_str() {
            "iota" => Self::Iota,
            "smr" | "shimmer" => Self::Shimmer,
            e => bail!("unsupported snapshot url: {e}"),
        })
    }
}

impl SnapshotUrl {
    /// Returns the Iota or Shimmer object snapshot download URL.
    pub fn to_url(&self) -> Url {
        match self {
            Self::Iota => Url::parse(IOTA_OBJECT_SNAPSHOT_URL).expect("should be valid URL"),
            Self::Shimmer => Url::parse(SHIMMER_OBJECT_SNAPSHOT_URL).expect("should be valid URL"),
            Self::Test(url) => url.clone(),
        }
    }

    /// Convert a gzip decoder to read the compressed object snapshot from S3.
    pub fn to_reader(&self) -> anyhow::Result<impl Read> {
        Ok(GzDecoder::new(BufReader::new(reqwest::blocking::get(
            self.to_url(),
        )?)))
    }
}
