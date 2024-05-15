// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crate::key_identity::KeyIdentity;
use crate::keytool::read_authority_keypair_from_file;
use crate::keytool::read_keypair_from_file;
use crate::keytool::CommandOutput;

use super::write_keypair_to_file;
use super::KeyToolCommand;
use anyhow::Ok;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::encoding::Base64;
use fastcrypto::encoding::Encoding;
use fastcrypto::encoding::Hex;
use fastcrypto::traits::ToFromBytes;
use rand::rngs::StdRng;
use rand::SeedableRng;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentScope;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, InMemKeystore, Keystore};
use sui_types::base_types::ObjectDigest;
use sui_types::base_types::ObjectID;
use sui_types::base_types::SequenceNumber;
use sui_types::base_types::SuiAddress;
use sui_types::crypto::get_key_pair;
use sui_types::crypto::get_key_pair_from_rng;
use sui_types::crypto::AuthorityKeyPair;
use sui_types::crypto::Ed25519SuiSignature;
use sui_types::crypto::EncodeDecodeBase64;
use sui_types::crypto::Secp256k1SuiSignature;
use sui_types::crypto::Secp256r1SuiSignature;
use sui_types::crypto::Signature;
use sui_types::crypto::SignatureScheme;
use sui_types::crypto::SuiKeyPair;
use sui_types::crypto::SuiSignatureInner;
use sui_types::transaction::TransactionData;
use sui_types::transaction::TEST_ONLY_GAS_UNIT_FOR_TRANSFER;
use tempfile::TempDir;
use tokio::test;

const TEST_MNEMONIC: &str = "result crisp session latin must fruit genuine question prevent start coconut brave speak student dismiss";

#[test]
async fn test_addresses_command() -> Result<(), anyhow::Error> {
    // Add 3 Ed25519 KeyPairs as default
    let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(3));

    // Add another 3 Secp256k1 KeyPairs
    for _ in 0..3 {
        keystore.add_key(None, SuiKeyPair::Secp256k1(get_key_pair().1))?;
    }

    // List all addresses with flag
    KeyToolCommand::List {
        sort_by_alias: true,
    }
    .execute(&mut keystore)
    .await
    .unwrap();
    Ok(())
}

#[test]
async fn test_flag_in_signature_and_keypair() -> Result<(), anyhow::Error> {
    let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));

    keystore.add_key(None, SuiKeyPair::Secp256k1(get_key_pair().1))?;
    keystore.add_key(None, SuiKeyPair::Ed25519(get_key_pair().1))?;

    for pk in keystore.keys() {
        let pk1 = pk.clone();
        let sig = keystore.sign_secure(&(&pk).into(), b"hello", Intent::sui_transaction())?;
        match sig {
            Signature::Ed25519SuiSignature(_) => {
                // signature contains corresponding flag
                assert_eq!(
                    *sig.as_ref().first().unwrap(),
                    Ed25519SuiSignature::SCHEME.flag()
                );
                // keystore stores pubkey with corresponding flag
                assert!(pk1.flag() == Ed25519SuiSignature::SCHEME.flag())
            }
            Signature::Secp256k1SuiSignature(_) => {
                assert_eq!(
                    *sig.as_ref().first().unwrap(),
                    Secp256k1SuiSignature::SCHEME.flag()
                );
                assert!(pk1.flag() == Secp256k1SuiSignature::SCHEME.flag())
            }
            Signature::Secp256r1SuiSignature(_) => {
                assert_eq!(
                    *sig.as_ref().first().unwrap(),
                    Secp256r1SuiSignature::SCHEME.flag()
                );
                assert!(pk1.flag() == Secp256r1SuiSignature::SCHEME.flag())
            }
        }
    }
    Ok(())
}

#[test]
async fn test_read_write_keystore_with_flag() {
    let dir = tempfile::TempDir::new().unwrap();

    // create Secp256k1 keypair
    let kp_secp = SuiKeyPair::Secp256k1(get_key_pair().1);
    let addr_secp: SuiAddress = (&kp_secp.public()).into();
    let fp_secp = dir.path().join(format!("{}.key", addr_secp));
    let fp_secp_2 = fp_secp.clone();

    // write Secp256k1 keypair to file
    let res = write_keypair_to_file(&kp_secp, &fp_secp);
    assert!(res.is_ok());

    // read from file as enum KeyPair success
    let kp_secp_read = read_keypair_from_file(fp_secp);
    assert!(kp_secp_read.is_ok());

    // KeyPair wrote into file is the same as read
    assert_eq!(
        kp_secp_read.unwrap().public().as_ref(),
        kp_secp.public().as_ref()
    );

    // read as AuthorityKeyPair fails
    let kp_secp_read = read_authority_keypair_from_file(fp_secp_2);
    assert!(kp_secp_read.is_err());

    // create Ed25519 keypair
    let kp_ed = SuiKeyPair::Ed25519(get_key_pair().1);
    let addr_ed: SuiAddress = (&kp_ed.public()).into();
    let fp_ed = dir.path().join(format!("{}.key", addr_ed));
    let fp_ed_2 = fp_ed.clone();

    // write Ed25519 keypair to file
    let res = write_keypair_to_file(&kp_ed, &fp_ed);
    assert!(res.is_ok());

    // read from file as enum KeyPair success
    let kp_ed_read = read_keypair_from_file(fp_ed);
    assert!(kp_ed_read.is_ok());

    // KeyPair wrote into file is the same as read
    assert_eq!(
        kp_ed_read.unwrap().public().as_ref(),
        kp_ed.public().as_ref()
    );

    // read from file as AuthorityKeyPair success
    let kp_ed_read = read_authority_keypair_from_file(fp_ed_2);
    assert!(kp_ed_read.is_err());
}

#[test]
async fn test_sui_operations_config() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("sui.keystore");
    let path1 = path.clone();
    // This is the hardcoded keystore in sui-operation: https://github.com/MystenLabs/sui-operations/blob/af04c9d3b61610dbb36401aff6bef29d06ef89f8/docker/config/generate/static/sui.keystore
    // If this test fails, address hardcoded in sui-operations is likely needed be updated.
    let kp = SuiKeyPair::decode_base64("ANRj4Rx5FZRehqwrctiLgZDPrY/3tI5+uJLCdaXPCj6C").unwrap();
    let contents = vec![kp.encode_base64()];
    let res = std::fs::write(path, serde_json::to_string_pretty(&contents).unwrap());
    assert!(res.is_ok());
    let read = FileBasedKeystore::new(&path1);
    assert!(read.is_ok());
    assert_eq!(
        SuiAddress::from_str("bc14937ffd5874a57afa10edf2d267d8eaaaf61081d718d9ba19cae85c00c6e8")
            .unwrap(),
        read.unwrap().addresses()[0]
    );

    // This is the hardcoded keystore in sui-operation: https://github.com/MystenLabs/sui-operations/blob/af04c9d3b61610dbb36401aff6bef29d06ef89f8/docker/config/generate/static/sui-benchmark.keystore
    // If this test fails, address hardcoded in sui-operations is likely needed be updated.
    let path2 = temp_dir.path().join("sui-benchmark.keystore");
    let path3 = path2.clone();
    let kp = SuiKeyPair::decode_base64("APCWxPNCbgGxOYKeMfPqPmXmwdNVyau9y4IsyBcmC14A").unwrap();
    let contents = vec![kp.encode_base64()];
    let res = std::fs::write(path2, serde_json::to_string_pretty(&contents).unwrap());
    assert!(res.is_ok());
    let read = FileBasedKeystore::new(&path3);
    assert_eq!(
        SuiAddress::from_str("e988a8fb85944173237d287e98e542ae50c119c02644856ed8db17fe9f528b13")
            .unwrap(),
        read.unwrap().addresses()[0]
    );
}

#[test]
async fn test_load_keystore_err() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("sui.keystore");
    let path2 = path.clone();

    // write encoded AuthorityKeyPair without flag byte to file
    let kp: AuthorityKeyPair = get_key_pair_from_rng(&mut StdRng::from_seed([0; 32])).1;
    let contents = kp.encode_base64();
    let res = std::fs::write(path, contents);
    assert!(res.is_ok());

    // cannot load keypair due to missing flag
    assert!(FileBasedKeystore::new(&path2).is_err());
}

#[test]
async fn test_private_keys_import_export() -> Result<(), anyhow::Error> {
    // private key in Bech32, private key in Hex, private key in Base64, derived Sui address in Hex
    const TEST_CASES: &[(&str, &str, &str, &str)] = &[
        (
            "suiprivkey1qzwant3kaegmjy4qxex93s0jzvemekkjmyv3r2sjwgnv2y479pgsywhveae",
            "0x9dd9ae36ee51b912a0364c58c1f21333bcdad2d91911aa127226c512be285102",
            "AJ3ZrjbuUbkSoDZMWMHyEzO82tLZGRGqEnImxRK+KFEC",
            "0x80ebb793af9b40569ed2c3be16e5bd76358997e28cc35ff48681157224c8b038",
        ),
        (
            "suiprivkey1qrh2sjl88rze74hwjndw3l26dqyz63tea5u9frtwcsqhmfk9vxdlx8cpv0g",
            "0xeea84be738c59f56ee94dae8fd5a68082d4579ed38548d6ec4017da6c5619bf3",
            "AO6oS+c4xZ9W7pTa6P1aaAgtRXntOFSNbsQBfabFYZvz",
            "0xee64bf4dedc0d6d853e156b04b4adae937da0b549d740b9de26d0360813f8f1c",
        ),
        (
            "suiprivkey1qzg73qyvfz0wpnyectkl08nrhe4pgnu0vqx8gydu96qx7uj4wyr8gcrjlh3",
            "0x91e8808c489ee0cc99c2edf79e63be6a144f8f600c7411bc2e806f7255710674",
            "AJHogIxInuDMmcLt955jvmoUT49gDHQRvC6Ab3JVcQZ0",
            "0x3a3bf7803ccb4903e2d9c4c11f80a24fe0c57a3a298f4ebd52667849554021b5",
        ),
    ];
    // assert correctness
    for (private_key, private_key_hex, private_key_base64, address) in TEST_CASES {
        let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
        KeyToolCommand::Import {
            alias: None,
            input_string: private_key.to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: None,
        }
        .execute(&mut keystore)
        .await?;
        let kp = SuiKeyPair::decode(private_key).unwrap();
        let kp_from_hex = SuiKeyPair::Ed25519(
            Ed25519KeyPair::from_bytes(&Hex::decode(private_key_hex).unwrap()).unwrap(),
        );
        assert_eq!(kp, kp_from_hex);

        let kp_from_base64 = SuiKeyPair::decode_base64(private_key_base64).unwrap();
        assert_eq!(kp, kp_from_base64);

        let addr = SuiAddress::from_str(address).unwrap();
        assert_eq!(SuiAddress::from(&kp.public()), addr);
        assert!(keystore.addresses().contains(&addr));

        // Export output shows the private key in Bech32
        let output = KeyToolCommand::Export {
            key_identity: KeyIdentity::Address(addr),
        }
        .execute(&mut keystore)
        .await?;
        match output {
            CommandOutput::Export(exported) => {
                assert_eq!(exported.exported_private_key, private_key.to_string());
            }
            _ => panic!("unexpected output"),
        }
    }

    for (private_key, _, _, addr) in TEST_CASES {
        let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
        // assert failure when private key is malformed
        let output = KeyToolCommand::Import {
            alias: None,
            input_string: private_key[1..].to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: None,
        }
        .execute(&mut keystore)
        .await;
        assert!(output.is_err());

        // importing an hex encoded string should fail
        let output = KeyToolCommand::Import {
            alias: None,
            input_string: addr.to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: None,
        }
        .execute(&mut keystore)
        .await;
        assert!(output.is_err());
    }

    Ok(())
}

#[test]
async fn test_mnemonics_ed25519() -> Result<(), anyhow::Error> {
    // Test case matches with /mysten/sui/sdk/typescript/test/unit/cryptography/ed25519-keypair.test.ts
    const TEST_CASES: [[&str; 3]; 3] = [
        [
            "film crazy soon outside stand loop subway crumble thrive popular green nuclear struggle pistol arm wife phrase warfare march wheat nephew ask sunny firm", 
            "suiprivkey1qrqqxhsu3ndp96644fjk4z5ams5ulgmvprklngt2jhvg2ujn5w4q23vm2y8", 
            "0x9f8e5379678525edf768d7b507dc1ba9016fc4f0eac976ab7f74077d95fba312"
        ],
        [
            "require decline left thought grid priority false tiny gasp angle royal system attack beef setup reward aunt skill wasp tray vital bounce inflict level", 
            "suiprivkey1qqcxaf57fnenvflpacacaumf6vl0rt0edddhytanvzhkqhwnjk0zsawjy3x", 
            "0x862738192e40540e0a5c9a5aca636f53b0cd76b0a9bef3386e05647feb4914ac"
        ],
        [
            "organ crash swim stick traffic remember army arctic mesh slice swear summer police vast chaos cradle squirrel hood useless evidence pet hub soap lake", 
            "suiprivkey1qzq39vxzm0gq7l8dc5dj5allpuww4mavhwhg8mua4cl3lj2c3fvhcsjgphc", 
            "0x2391788ca49c7f0f00699bc2bad45f80c343b4d1df024285c132259433d7ff31"
        ]
    ];

    for t in TEST_CASES {
        let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
        KeyToolCommand::Import {
            alias: None,
            input_string: t[0].to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: None,
        }
        .execute(&mut keystore)
        .await?;
        let kp = SuiKeyPair::decode(t[1]).unwrap();
        let addr = SuiAddress::from_str(t[2]).unwrap();
        assert_eq!(SuiAddress::from(&kp.public()), addr);
        assert!(keystore.addresses().contains(&addr));
    }
    Ok(())
}

#[test]
async fn test_mnemonics_secp256k1() -> Result<(), anyhow::Error> {
    // Test case matches with /mysten/sui/sdk/typescript/test/unit/cryptography/secp256k1-keypair.test.ts
    const TEST_CASES: [[&str; 3]; 3] = [ 
        [
            "film crazy soon outside stand loop subway crumble thrive popular green nuclear struggle pistol arm wife phrase warfare march wheat nephew ask sunny firm", 
            "suiprivkey1q8cy2ll8a0dmzzzwn9zavrug0qf47cyuj6k2r4r6rnjtpjhrdh52vallxwz", 
            "0x8520d58dde1ab268349b9a46e5124ae6fe7e4c61df4ca2bc9c97d3c4d07b0b55"
        ],
        [
            "require decline left thought grid priority false tiny gasp angle royal system attack beef setup reward aunt skill wasp tray vital bounce inflict level", 
            "suiprivkey1q9hm330d05jcxfvmztv046p8kclyaj39hk6elqghgpq4sz4x23hk2jtdnjf", 
            "0x3740d570eefba29dfc0fdd5829848902064e31ecd059ca05c401907fa8646f61"
        ],
        [
            "organ crash swim stick traffic remember army arctic mesh slice swear summer police vast chaos cradle squirrel hood useless evidence pet hub soap lake", 
            "suiprivkey1qx2dnch6363h7gdqqfkzmmlequzj4ul3x4fq6dzyajk7wc2c0jgcxdv2dvl", 
            "0x943b852c37fef403047e06ff5a2fa216557a4386212fb29554babdd3e1899da5"
        ]
    ];

    for t in TEST_CASES {
        let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
        KeyToolCommand::Import {
            alias: None,
            input_string: t[0].to_string(),
            key_scheme: SignatureScheme::Secp256k1,
            derivation_path: None,
        }
        .execute(&mut keystore)
        .await?;
        let kp = SuiKeyPair::decode(t[1]).unwrap();
        let addr = SuiAddress::from_str(t[2]).unwrap();
        assert_eq!(SuiAddress::from(&kp.public()), addr);
        assert!(keystore.addresses().contains(&addr));
    }
    Ok(())
}

#[test]
async fn test_mnemonics_secp256r1() -> Result<(), anyhow::Error> {
    // Test case matches with /mysten/sui/sdk/typescript/test/unit/cryptography/secp256r1-keypair.test.ts
    const TEST_CASES: [[&str; 3]; 3] = [
        [
            "act wing dilemma glory episode region allow mad tourist humble muffin oblige",
            "suiprivkey1qtt65ua2lhal76zg4cxd6umdqynv2rj2gzrntp5rwlnyj370jg3pwhxg9kc",
            "0x779a63b28528210a5ec6c4af5a70382fa3f0c2d3f98dcbe4e3a4ae2f8c39cc9c",
        ],
        [
            "flag rebel cabbage captain minimum purpose long already valley horn enrich salt",
            "suiprivkey1qtcjgmue7q8u4gtutfvfpx3zj3aa2r9pqssuusrltxfv68eqhzsgjyhk7e4",
            "0x8b45523042933aa55f57e2ccc661304baed292529b6e67a0c9857c1f3f871806",
        ],
        [
            "area renew bar language pudding trial small host remind supreme cabbage era",
            "suiprivkey1qtxafg26qxeqy7f56gd2rvsup0a5kl4cre7nt2rtcrf0p3v5pwd4c595zjp",
            "0x8528ef86150ec331928a8b3edb8adbe2fb523db8c84679aa57a931da6a4cdb25",
        ],
    ];

    for [mnemonics, sk, address] in TEST_CASES {
        let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
        KeyToolCommand::Import {
            alias: None,
            input_string: mnemonics.to_string(),
            key_scheme: SignatureScheme::Secp256r1,
            derivation_path: None,
        }
        .execute(&mut keystore)
        .await?;

        let kp = SuiKeyPair::decode(sk).unwrap();
        let addr = SuiAddress::from_str(address).unwrap();
        assert_eq!(SuiAddress::from(&kp.public()), addr);
        assert!(keystore.addresses().contains(&addr));
    }

    Ok(())
}

#[test]
async fn test_invalid_derivation_path() -> Result<(), anyhow::Error> {
    let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
    assert!(KeyToolCommand::Import {
        alias: None,
        input_string: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/44'/1'/0'/0/0".parse().unwrap()),
    }
    .execute(&mut keystore)
    .await
    .is_err());

    assert!(KeyToolCommand::Import {
        alias: None,
        input_string: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/0'/4218'/0'/0/0".parse().unwrap()),
    }
    .execute(&mut keystore)
    .await
    .is_err());

    assert!(KeyToolCommand::Import {
        alias: None,
        input_string: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/54'/4218'/0'/0/0".parse().unwrap()),
    }
    .execute(&mut keystore)
    .await
    .is_err());

    assert!(KeyToolCommand::Import {
        alias: None,
        input_string: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::Secp256k1,
        derivation_path: Some("m/54'/4218'/0'/0'/0'".parse().unwrap()),
    }
    .execute(&mut keystore)
    .await
    .is_err());

    assert!(KeyToolCommand::Import {
        alias: None,
        input_string: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::Secp256k1,
        derivation_path: Some("m/44'/4218'/0'/0/0".parse().unwrap()),
    }
    .execute(&mut keystore)
    .await
    .is_err());

    Ok(())
}

#[test]
async fn test_valid_derivation_path() -> Result<(), anyhow::Error> {
    let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
    assert!(KeyToolCommand::Import {
        alias: None,
        input_string: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/44'/4218'/0'/0'/0'".parse().unwrap()),
    }
    .execute(&mut keystore)
    .await
    .is_ok());

    assert!(KeyToolCommand::Import {
        alias: None,
        input_string: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/44'/4218'/0'/0'/1'".parse().unwrap()),
    }
    .execute(&mut keystore)
    .await
    .is_ok());

    assert!(KeyToolCommand::Import {
        alias: None,
        input_string: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/44'/4218'/1'/0'/1'".parse().unwrap()),
    }
    .execute(&mut keystore)
    .await
    .is_ok());

    assert!(KeyToolCommand::Import {
        alias: None,
        input_string: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::Secp256k1,
        derivation_path: Some("m/54'/4218'/0'/0/1".parse().unwrap()),
    }
    .execute(&mut keystore)
    .await
    .is_ok());

    assert!(KeyToolCommand::Import {
        alias: None,
        input_string: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::Secp256k1,
        derivation_path: Some("m/54'/4218'/1'/0/1".parse().unwrap()),
    }
    .execute(&mut keystore)
    .await
    .is_ok());
    Ok(())
}

#[test]
async fn test_keytool_bls12381() -> Result<(), anyhow::Error> {
    let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
    KeyToolCommand::Generate {
        key_scheme: SignatureScheme::BLS12381,
        derivation_path: None,
        word_length: None,
    }
    .execute(&mut keystore)
    .await?;
    Ok(())
}

#[test]
async fn test_sign_command() -> Result<(), anyhow::Error> {
    // Add a keypair
    let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(1));
    let binding = keystore.addresses();
    let sender = binding.first().unwrap();
    let alias = keystore.get_alias_by_address(sender).unwrap();

    // Create a dummy TransactionData
    let gas = (
        ObjectID::random(),
        SequenceNumber::new(),
        ObjectDigest::random(),
    );
    let gas_price = 1;
    let tx_data = TransactionData::new_pay_sui(
        *sender,
        vec![gas],
        vec![SuiAddress::random_for_testing_only()],
        vec![10000],
        gas,
        gas_price * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        gas_price,
    )
    .unwrap();

    // Sign an intent message for the transaction data and a passed-in intent with scope as PersonalMessage.
    KeyToolCommand::Sign {
        address: KeyIdentity::Address(*sender),
        data: Base64::encode(bcs::to_bytes(&tx_data)?),
        intent: Some(Intent::sui_app(IntentScope::PersonalMessage)),
    }
    .execute(&mut keystore)
    .await?;

    // Sign an intent message for the transaction data without intent passed in, so default is used.
    KeyToolCommand::Sign {
        address: KeyIdentity::Address(*sender),
        data: Base64::encode(bcs::to_bytes(&tx_data)?),
        intent: None,
    }
    .execute(&mut keystore)
    .await?;

    // Sign an intent message for the transaction data without intent passed in, so default is used.
    // Use alias for signing instead of the address
    KeyToolCommand::Sign {
        address: KeyIdentity::Alias(alias),
        data: Base64::encode(bcs::to_bytes(&tx_data)?),
        intent: None,
    }
    .execute(&mut keystore)
    .await?;
    Ok(())
}
