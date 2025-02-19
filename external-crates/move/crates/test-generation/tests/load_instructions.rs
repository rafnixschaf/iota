// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

extern crate test_generation;
use move_binary_format::file_format::{Bytecode, ConstantPoolIndex, SignatureToken};
use test_generation::abstract_state::{AbstractState, AbstractValue};

mod common;

#[test]
fn bytecode_ldu64() {
    let state1 = AbstractState::new();
    let (state2, _) = common::run_instruction(Bytecode::LdU64(0), state1);
    assert_eq!(
        state2.stack_peek(0),
        Some(AbstractValue::new_primitive(SignatureToken::U64)),
        "stack type postcondition not met"
    );
}

#[test]
fn bytecode_ldtrue() {
    let state1 = AbstractState::new();
    let (state2, _) = common::run_instruction(Bytecode::LdTrue, state1);
    assert_eq!(
        state2.stack_peek(0),
        Some(AbstractValue::new_primitive(SignatureToken::Bool)),
        "stack type postcondition not met"
    );
}

#[test]
fn bytecode_ldfalse() {
    let state1 = AbstractState::new();
    let (state2, _) = common::run_instruction(Bytecode::LdFalse, state1);
    assert_eq!(
        state2.stack_peek(0),
        Some(AbstractValue::new_primitive(SignatureToken::Bool)),
        "stack type postcondition not met"
    );
}

#[test]
fn bytecode_ldconst() {
    let state1 = AbstractState::new();
    let (state2, _) = common::run_instruction(Bytecode::LdConst(ConstantPoolIndex::new(0)), state1);
    assert_eq!(
        state2.stack_peek(0),
        Some(AbstractValue::new_primitive(SignatureToken::Address)),
        "stack type postcondition not met"
    );
}
