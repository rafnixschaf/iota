// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaClient, IotaObjectData } from '@iota/iota-sdk/client';
import { Transaction } from '@iota/iota-sdk/transactions';
import { STARDUST_PACKAGE_ID } from '../../constants/migration.constants';
import { z } from 'zod';
import { IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';

type NestedResultType = {
    $kind: 'NestedResult';
    NestedResult: [number, number];
};

const ExpirationUnlockConditionSchema = z.object({
    type: z.string(),
    fields: z.object({
        owner: z.string(),
        return_address: z.string(),
        unix_time: z.number(),
    }),
});

const StorageDepositReturnUnlockConditionSchema = z.object({
    type: z.string(),
    fields: z.object({
        return_address: z.string(),
        return_amount: z.string(),
    }),
});

const TimelockUnlockConditionSchema = z.object({
    type: z.string(),
    fields: z.object({
        unix_time: z.number(),
    }),
});

const CommonOutputObjectSchema = z.object({
    id: z.object({
        id: z.string(),
    }),
    balance: z.string(),
    native_tokens: z.object({
        type: z.string(),
        fields: z.object({
            id: z.object({
                id: z.string(),
            }),
            size: z.string(),
        }),
    }),
});

const CommonOutputObjectWithUcSchema = CommonOutputObjectSchema.extend({
    expiration_uc: ExpirationUnlockConditionSchema.nullable().optional(),
    storage_deposit_return_uc: StorageDepositReturnUnlockConditionSchema.nullable().optional(),
    timelock_uc: TimelockUnlockConditionSchema.nullable().optional(),
});

const BasicOutputObjectSchema = CommonOutputObjectWithUcSchema.extend({
    metadata: z.array(z.number()).nullable().optional(),
    tag: z.array(z.number()).nullable().optional(),
    sender: z.string().nullable().optional(),
});

const NftOutputObjectSchema = CommonOutputObjectWithUcSchema;

export type ExpirationUnlockCondition = z.infer<typeof ExpirationUnlockConditionSchema>;
export type StorageDepositReturnUnlockCondition = z.infer<
    typeof StorageDepositReturnUnlockConditionSchema
>;
export type TimelockUnlockCondition = z.infer<typeof TimelockUnlockConditionSchema>;
export type CommonOutputObject = z.infer<typeof CommonOutputObjectSchema>;
export type CommonOutputObjectWithUc = z.infer<typeof CommonOutputObjectWithUcSchema>;
export type BasicOutputObject = z.infer<typeof BasicOutputObjectSchema>;
export type NftOutputObject = z.infer<typeof NftOutputObjectSchema>;

export async function getNativeTokenTypesFromBag(
    bagId: string,
    client: IotaClient,
): Promise<string[]> {
    const nativeTokenDynamicFields = await client.getDynamicFields({
        parentId: bagId,
    });
    const nativeTokenTypes: string[] = [];
    for (const nativeToken of nativeTokenDynamicFields.data) {
        nativeTokenTypes.push(nativeToken?.name?.value as string);
    }

    return nativeTokenTypes;
}

export function validateBasicOutputObject(outputObject: IotaObjectData): BasicOutputObject {
    if (outputObject.content?.dataType !== 'moveObject') {
        throw new Error('Invalid basic output object');
    }
    const result = BasicOutputObjectSchema.safeParse(outputObject.content.fields);
    if (!result.success) {
        throw new Error('Invalid basic output object content');
    }
    return result.data;
}

export function validateNftOutputObject(outputObject: IotaObjectData): NftOutputObject {
    if (outputObject.content?.dataType !== 'moveObject') {
        throw new Error('Invalid nft output object');
    }
    const result = NftOutputObjectSchema.safeParse(outputObject.content.fields);
    if (!result.success) {
        throw new Error('Invalid nft output object content');
    }
    return result.data;
}

export async function createMigrationTransaction(
    client: IotaClient,
    address: string,
    basicOutputs: IotaObjectData[] = [],
    nftOutputs: IotaObjectData[] = [],
): Promise<Transaction> {
    const ptb = new Transaction();

    const coinsFromBasicOutputs: NestedResultType[] = [];

    // Basic Outputs
    for (const basicOutputObject of basicOutputs) {
        const validatedOutputObject = validateBasicOutputObject(basicOutputObject);
        const basicOutputObjectId = validatedOutputObject.id.id;
        const bagId = validatedOutputObject.native_tokens.fields.id.id;
        const bagSize = validatedOutputObject.native_tokens.fields.size;
        const nativeTokenTypes: string[] =
            Number(bagSize) > 0 ? await getNativeTokenTypesFromBag(bagId, client) : [];

        const migratableResult = ptb.moveCall({
            target: `${STARDUST_PACKAGE_ID}::basic_output::extract_assets`,
            typeArguments: [IOTA_TYPE_ARG],
            arguments: [ptb.object(basicOutputObjectId)],
        });

        const balance = migratableResult[0];
        let nativeTokensBag = migratableResult[1];

        // Convert Balance in Coin
        const [coin] = ptb.moveCall({
            target: '0x02::coin::from_balance',
            typeArguments: [IOTA_TYPE_ARG],
            arguments: [ptb.object(balance)],
        });

        coinsFromBasicOutputs.push(coin);

        for (const nativeTokenType of nativeTokenTypes) {
            [nativeTokensBag] = ptb.moveCall({
                target: '0x107a::utilities::extract_and_send_to',
                typeArguments: [nativeTokenType],
                arguments: [ptb.object(nativeTokensBag), ptb.pure.address(address)],
            });
        }

        ptb.moveCall({
            target: '0x02::bag::destroy_empty',
            arguments: [ptb.object(nativeTokensBag)],
        });
    }

    // NFT Outputs
    const coinsFromNftOutputs: NestedResultType[] = [];
    const nftsFromNftOutputs: NestedResultType[] = [];

    for (const nftOutputObject of nftOutputs) {
        const validatedOutputObject = validateNftOutputObject(nftOutputObject);
        const nftOutputObjectId = validatedOutputObject.id.id;
        const bagId = validatedOutputObject.native_tokens.fields.id.id;
        const bagSize = validatedOutputObject.native_tokens.fields.size;
        const nativeTokenTypes: string[] =
            Number(bagSize) > 0 ? await getNativeTokenTypesFromBag(bagId, client) : [];

        const migratableResult = ptb.moveCall({
            target: `${STARDUST_PACKAGE_ID}::nft_output::extract_assets`,
            typeArguments: [IOTA_TYPE_ARG],
            arguments: [ptb.object(nftOutputObjectId)],
        });

        const balance = migratableResult[0];
        let nativeTokensBag = migratableResult[1];
        const nft = migratableResult[2];

        nftsFromNftOutputs.push(nft);

        // Convert Balance in Coin
        const [coin] = ptb.moveCall({
            target: '0x02::coin::from_balance',
            typeArguments: [IOTA_TYPE_ARG],
            arguments: [ptb.object(balance)],
        });
        coinsFromNftOutputs.push(coin);

        for (const nativeTokenType of nativeTokenTypes) {
            [nativeTokensBag] = ptb.moveCall({
                target: '0x107a::utilities::extract_and_send_to',
                typeArguments: [nativeTokenType],
                arguments: [ptb.object(nativeTokensBag), ptb.pure.address(address)],
            });
        }

        ptb.moveCall({
            target: '0x02::bag::destroy_empty',
            arguments: [ptb.object(nativeTokensBag)],
        });
    }

    const coinOne = coinsFromBasicOutputs.shift() || coinsFromNftOutputs.shift();
    const remainingCoins = [...coinsFromBasicOutputs, ...coinsFromNftOutputs];
    if (coinOne) {
        if (remainingCoins.length > 0) {
            ptb.mergeCoins(coinOne, remainingCoins);
        }
        ptb.transferObjects([coinOne, ...nftsFromNftOutputs], ptb.pure.address(address));
    }

    return ptb;
}
