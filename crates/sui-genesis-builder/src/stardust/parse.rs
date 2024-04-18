//! Types and logic to parse a full Stardust snapshot.
use std::{
    fs::File,
    io::{BufRead, Read},
    path::Path,
};

use anyhow::Result;
use iota_sdk::types::block::{output::Output, protocol::ProtocolParameters};
use packable::PackableExt;

use super::types::{FullSnapshotHeader, OutputHeader, TOTAL_SUPPLY_IOTA};

/// Parse a full snapshot file located at `path` and iterate over the
/// recorded [`Output`] values.
pub fn parse_full_snapshot(path: impl AsRef<Path>) -> Result<()> {
    let snapshot_file = File::open(path)?;
    let mut reader = std::io::BufReader::new(snapshot_file);
    let mut buf = [0_u8; FullSnapshotHeader::LENGTH];
    reader.read_exact(&mut buf)?;

    let full_header = FullSnapshotHeader::unpack_verified(buf.as_slice(), &())?;

    println!("Output count: {}", full_header.output_count());

    let total_supply = iterate_on_outputs(&mut reader, full_header.output_count())
        .try_fold(0, |acc, output| {
            Ok::<_, anyhow::Error>(acc + output?.amount())
        })?;
    assert_eq!(total_supply, TOTAL_SUPPLY_IOTA);
    println!("Total supply: {total_supply}");
    Ok(())
}

fn iterate_on_outputs(
    src: &mut impl BufRead,
    output_count: u64,
) -> impl Iterator<Item = Result<Output, anyhow::Error>> + '_ {
    let mut header_buf = [0_u8; OutputHeader::LENGTH];
    let mut output_buf = [0_u8; u16::MAX as usize];

    (0..output_count).map(move |_| {
        src.read_exact(&mut header_buf)?;
        let header = OutputHeader::unpack_verified(header_buf.as_slice(), &())?;
        src.read_exact(&mut output_buf[0..header.length() as usize])?;
        let output = Output::unpack_verified(
            &output_buf[0..header.length() as usize],
            &ProtocolParameters::default(),
        )?;
        Ok(output)
    })
}
