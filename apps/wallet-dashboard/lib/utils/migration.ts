// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { CommonOutputObjectWithUc } from '@iota/core';
import { IotaObjectData } from '@iota/iota-sdk/client';

export type StardustMigrationGroupedObjects = {
    migratable: IotaObjectData[];
    unmigratable: IotaObjectData[];
};

export function groupStardustObjectsByMigrationStatus(
    stardustOutputObjects: IotaObjectData[],
    epochTimestamp: number,
    address: string,
): StardustMigrationGroupedObjects {
    const migratable: IotaObjectData[] = [];
    const unmigratable: IotaObjectData[] = [];

    const epochUnix = epochTimestamp / 1000;

    for (const outputObject of stardustOutputObjects) {
        if (!outputObject) {
            unmigratable.push(outputObject);
            continue;
        }
        const outputObjectFields = (
            outputObject.content as unknown as {
                fields: CommonOutputObjectWithUc;
            }
        ).fields;

        if (outputObjectFields.expiration_uc) {
            unmigratable.push(outputObject);
            continue;
            const unlockableAddress =
                outputObjectFields.expiration_uc.fields.unix_time <= epochUnix
                    ? outputObjectFields.expiration_uc.fields.return_address
                    : outputObjectFields.expiration_uc.fields.owner;
            if (unlockableAddress !== address) {
                unmigratable.push(outputObject);
                continue;
            }
        }
        if (
            outputObjectFields.timelock_uc &&
            outputObjectFields.timelock_uc.fields.unix_time > epochUnix
        ) {
            unmigratable.push(outputObject);
            continue;
        }

        migratable.push(outputObject);
    }

    return { migratable, unmigratable };
}
