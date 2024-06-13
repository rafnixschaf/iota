// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::{Display, Formatter};

use tabled::{
    builder::Builder as TableBuilder,
    settings::{Panel as TablePanel, Style as TableStyle},
};

use crate::{
    client_ptb::{
        ast::{GAS_BUDGET, GAS_COIN, JSON, SUMMARY, WARN_SHADOWS},
        ptb::PTBPreview,
    },
    sp, HORIZONTAL_LINE,
};

impl<'a> Display for PTBPreview<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = TableBuilder::default();
        builder.push_record(["command", "values"]);
        for sp!(_, cmd) in &self.program.commands {
            if let Some((command, vals)) = cmd.to_string().split_once(' ') {
                builder.push_record([command, vals]);
            }
        }
        // index of horizontal line to draw after commands
        let line_index = builder.count_records();
        builder.push_record([
            GAS_BUDGET,
            self.program_metadata.gas_budget.value.to_string().as_str(),
        ]);
        if let Some(gas_coin_id) = self.program_metadata.gas_object_id {
            builder.push_record([GAS_COIN, gas_coin_id.value.to_string().as_str()]);
        }
        if self.program_metadata.json_set {
            builder.push_record([JSON, "true"]);
        }
        if self.program_metadata.summary_set {
            builder.push_record([SUMMARY, "true"]);
        }
        if self.program.warn_shadows_set {
            builder.push_record([WARN_SHADOWS, "true"]);
        }
        // while theoretically it cannot happen because parsing the PTB requires at
        // least a gas-budget which leads to having at least 1 row,
        // check that there are actual rows in the table
        if builder.count_records() < 1 {
            return write!(f, "PTB is empty.");
        }
        let mut table = builder.build();
        table.with(TablePanel::header("PTB Preview"));
        table.with(TableStyle::rounded().horizontals([
            (1, HORIZONTAL_LINE),
            (2, HORIZONTAL_LINE),
            (3, HORIZONTAL_LINE),
            (line_index + 2, HORIZONTAL_LINE),
        ]));
        table.with(tabled::settings::style::BorderSpanCorrection);

        write!(f, "{}", table)
    }
}
