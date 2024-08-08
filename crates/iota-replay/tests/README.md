`cargo test`

currently fails as the json files contain a state that's not valid with our changes

output with the added println! in replay.rs

```
(V2(TransactionEffectsV2 { status: Failure { error: VMInvariantViolation, command: None }, executed_epoch: 158, gas_used: GasCostSummary { computation_cost: 750000, storage_cost: 2622000, storage_rebate: 2622000, non_refundable_storage_fee: 0 }, transaction_digest: TransactionDigest(4fxF3X4N8D6JGByWdbJvCCv1iuoT8GSEhXgPHJnzNHWP), gas_object_index: Some(0), events_digest: None, dependencies: [TransactionDigest(3fqrGiknVvk82qum6zLYR749UTTyavaoH8TLU6bym8di), TransactionDigest(5rr1KbGr5ht2kWrPwgArYuaYsauoDG8CpuJLnaKnkgL6), TransactionDigest(7SuYS2Zphhb7KFnoanQBArUmXUQjKhehZ5fQkET4wNcN), TransactionDigest(ACKQVv64dXCmpxE1QPU2QMLxVpmtbzrFGMtWYY6szJrQ), TransactionDigest(BEb8wBMP1GawREPj4XMmFC75VTmZL5U6aMLLo7122eWn)], lamport_version: SequenceNumber(26932893), changed_objects: [(0xb31edde13c4b6b512c17ebbfe6e3870bd38abe9197cdcf801ef604e713610fa1, EffectsObjectChange { input_state: Exist(((SequenceNumber(26932892), o#Ex867bCQkeiTCvcRqhkbAtZWPyVPcMbNRj1acbfyZdAs), AddressOwner(0xad5b541177c80832e38054522eb4d1030b43262d5c5d0f0ee2eb034642418c57))), output_state: ObjectWrite((o#7yMzXJqCh1Bu8m9CxhFDrC15Q68ZSDDPdyDYiSRe6yws, AddressOwner(0xad5b541177c80832e38054522eb4d1030b43262d5c5d0f0ee2eb034642418c57))), id_operation: None }), (0xdba1b40f3537441b51d2848fc0a149610e48e67c1cc48c6ad641767622000623, EffectsObjectChange { input_state: Exist(((SequenceNumber(10941462), o#97EMwmR1yvKzotocnPsNs5gLiHPkDztrM5WsVeE9d1y), AddressOwner(0xad5b541177c80832e38054522eb4d1030b43262d5c5d0f0ee2eb034642418c57))), output_state: ObjectWrite((o#EVip2GKbBqAV6q9CJ4UdUTUC2hq35rSRLhLHD4eoN7wF, AddressOwner(0xad5b541177c80832e38054522eb4d1030b43262d5c5d0f0ee2eb034642418c57))), id_operation: None })], unchanged_shared_objects: [], aux_data_digest: None }), Some(ExecutionError { inner: ExecutionErrorInner { kind: VMInvariantViolation, source: Some(VMError { major_status: UNEXPECTED_DESERIALIZATION_ERROR, sub_status: None, message: Some("Unexpected verifier/deserialization error! This likely means there is code stored on chain that is unverifiable!\nError: VMError { major_status: CODE_DESERIALIZATION_ERROR, sub_status: None, message: Some(\"Deserialization error: PartialVMError { major_status: MALFORMED, sub_status: None, message: Some(\\\"Bad Identifier pool size\\\"), exec_state: None, indices: [], offsets: [] }\"), exec_state: None, location: Module(ModuleId { address: 0000000000000000000000000000000000000000000000000000000000000002, name: Identifier(\"balance\") }), indices: [], offsets: [] }"), exec_state: None, location: Module(ModuleId { address: 0000000000000000000000000000000000000000000000000000000000000002, name: Identifier("balance") }), indices: [], offsets: [] }), command: None } }))
On-chain vs local diff
thread 'replay_sandboxes' panicked at crates/iota-replay/tests/regression_replay.rs:24:14:
called `Result::unwrap()` on an `Err` value: EffectsForked: Effects for digest 4fxF3X4N8D6JGByWdbJvCCv1iuoT8GSEhXgPHJnzNHWP forked with diff 
   V1(
       IotaTransactionBlockEffectsV1 {
---        status: Success,
+++        status: Failure {
+++            error: "VMInvariantViolation",
+++        },
           executed_epoch: 158,
           gas_used: GasCostSummary {
               computation_cost: 750000,
               storage_cost: 2622000,
               storage_rebate: 2622000,
               non_refundable_storage_fee: 0,
           },
           modified_at_versions: [
               IotaTransactionBlockEffectsModifiedAtVersions {
                   object_id: 0xb31edde13c4b6b512c17ebbfe6e3870bd38abe9197cdcf801ef604e713610fa1,
                   sequence_number: SequenceNumber(
                       26932892,
                   ),
               },
               IotaTransactionBlockEffectsModifiedAtVersions {
                   object_id: 0xdba1b40f3537441b51d2848fc0a149610e48e67c1cc48c6ad641767622000623,
                   sequence_number: SequenceNumber(
                       10941462,
                   ),
               },
           ],
           shared_objects: [],
           transaction_digest: TransactionDigest(
               4fxF3X4N8D6JGByWdbJvCCv1iuoT8GSEhXgPHJnzNHWP,
           ),
---        created: [
---            OwnedObjectRef {
---                owner: Immutable,
---                reference: IotaObjectRef {
---                    object_id: 0x0440aedc27c9a57ea357bd8fe1525a00d60d7f442a380924a7e7c1d79853bb8b,
---                    version: SequenceNumber(
---                        5,
---                    ),
---                    digest: o#Gm4LuFuwBvgmBuLSMAxEp2zkp1RLsfgwzv8xdscmzowH,
---                },
---            },
---        ],
+++        created: [],
```
