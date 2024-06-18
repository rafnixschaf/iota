// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Example demonstrating parsing Stardust UTXOs from a snapshot file
//! and finding assets trapped in ownership cycles.
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
};

use iota_genesis_builder::stardust::parse::FullSnapshotParser;
use iota_sdk::types::block::{address::Address, output::Output};

fn main() -> anyhow::Result<()> {
    let Some(path) = std::env::args().nth(1) else {
        anyhow::bail!("please provide path to the full-snapshot file");
    };
    let file = File::open(path)?;

    let parser = FullSnapshotParser::new(file)?;
    println!("Output count: {}", parser.header.output_count());

    let mut ownership_map: HashMap<Address, HashSet<Address>> = HashMap::new();

    #[derive(Debug, Copy, Clone, Default)]
    struct OutputStat {
        nft: usize,
        alias: usize,
        other: usize,
    }

    impl OutputStat {
        fn inc_nft(&mut self) {
            self.nft += 1;
        }
        fn inc_alias(&mut self) {
            self.alias += 1;
        }
        fn inc_other(&mut self) {
            self.other += 1;
        }
    }

    let stat = parser
        .outputs()
        .try_fold(OutputStat::default(), |mut acc, output| {
            let (header, output) = output?;
            match output {
                Output::Alias(alias) => {
                    acc.inc_alias();
                    let owner = alias.governor_address().clone();
                    let owned = alias.alias_address(&header.output_id()).into();
                    ownership_map.entry(owner).or_default().insert(owned);
                }
                Output::Nft(nft) => {
                    acc.inc_nft();
                    let owner = nft.address().clone();
                    let owned = nft.nft_address(&header.output_id()).into();
                    ownership_map.entry(owner).or_default().insert(owned);
                }
                _ => acc.inc_other(),
            }
            Ok::<_, anyhow::Error>(acc)
        })?;

    println!("{stat:?}");
    println!("NFT/Alias owner count: {}", ownership_map.len());

    // find cycles
    let mut trapped_assets = Vec::new();
    for parent in ownership_map.keys() {
        let mut visit = VecDeque::new();
        let mut chain = VecDeque::new();
        let mut descending = false;
        visit.push_back(parent.clone());
        'traversal: while !visit.is_empty() {
            let parent = visit.pop_back().unwrap();
            chain.push_back(parent.clone());
            let children = ownership_map.get(&parent).unwrap();

            for child in children {
                if ownership_map.contains_key(child) {
                    if !chain.contains(child) {
                        visit.push_back(child.clone());
                        descending = true;
                    } else {
                        chain.push_back(child.clone());
                        println!(
                            "Detected a cycle! Tried to add {child} which is also a parent. Cycle:\n{chain:#?}"
                        );
                        trapped_assets.push(child.clone());
                        break 'traversal;
                    }
                }
            }

            if !descending {
                chain.pop_back();
            }
        }
    }

    println!("Trapped assets:\n{trapped_assets:#?}");

    Ok(())
}
