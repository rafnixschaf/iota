//! Types and logic to parse a full Stardust snapshot.
use std::io::{BufReader, Read};

use anyhow::Result;
use iota_sdk::types::block::{output::Output, protocol::ProtocolParameters};
use packable::PackableExt;

use super::types::{FullSnapshotHeader, OutputHeader};

/// Parse a full-snapshot using a [`BufReader`] internally.
pub struct FullSnapshotParser<R: Read> {
    reader: BufReader<R>,
    /// The full-snapshot header
    pub header: FullSnapshotHeader,
}

impl<R: Read> FullSnapshotParser<R> {
    pub fn new(reader: R) -> Result<Self> {
        let mut reader = std::io::BufReader::new(reader);
        let mut buf = [0_u8; FullSnapshotHeader::LENGTH];
        reader.read_exact(&mut buf)?;

        let header = FullSnapshotHeader::unpack_verified(buf.as_slice(), &())?;
        Ok(Self { reader, header })
    }

    /// Provide an iterator over the Stardust UTXOs recorded in the snapshot.
    pub fn outputs(mut self) -> impl Iterator<Item = Result<Output, anyhow::Error>> {
        let mut header_buf = [0_u8; OutputHeader::LENGTH];
        let mut output_buf = [0_u8; u16::MAX as usize];

        (0..self.header.output_count()).map(move |_| {
            self.reader.read_exact(&mut header_buf)?;
            let header = OutputHeader::unpack_verified(header_buf.as_slice(), &())?;
            let output_bytes = &mut output_buf[0..header.length() as usize];
            self.reader.read_exact(output_bytes)?;
            let output = Output::unpack_verified(output_bytes, &ProtocolParameters::default())?;
            Ok(output)
        })
    }
}
