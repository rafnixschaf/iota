// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { IotaMoveNormalizedType } from '@iota/iota-sdk/client';

/**
 * Converts a IotaMoveNormalizedType to string
 * @param param A parameter's normalized type of a function
 * @param functionTypeArgNames Parameters can be generic like 0x2::coin::Coin<T>.
 * T is provided on function level with the type_parameters field of IotaMoveNormalizedFunction that defines the abilities.
 * This parameter can be an array of strings that define the actual type or names like T1 that can be used to make the type of the parameter more specific. If
 * functionTypeArgNames or the index that the parameter expects are not defines then a default value T{index} is used.
 * @param str This function is recursive and this field is used to pass the already resolved type
 * @returns
 */
export function normalizedFunctionParameterTypeToString(
    param: IotaMoveNormalizedType,
    functionTypeArgNames?: string[],
    str = '',
): string {
    if (typeof param === 'string') {
        return str + param;
    }
    if ('TypeParameter' in param) {
        return str + (functionTypeArgNames?.[param.TypeParameter] ?? `T${param.TypeParameter}`);
    }
    if ('Reference' in param || 'MutableReference' in param) {
        const p = 'Reference' in param ? param.Reference : param.MutableReference;
        return normalizedFunctionParameterTypeToString(p, functionTypeArgNames, str);
    }
    if ('Vector' in param) {
        return (
            normalizedFunctionParameterTypeToString(
                param.Vector,
                functionTypeArgNames,
                `${str}Vector<`,
            ) + '>'
        );
    }
    if ('Struct' in param) {
        const theType = param.Struct;
        const theTypeArgs = theType.typeArguments;
        const theTypeArgsStr = theTypeArgs
            .map((aTypeArg) =>
                normalizedFunctionParameterTypeToString(aTypeArg, functionTypeArgNames),
            )
            .join(', ');
        return `${[theType.address, theType.module, theType.name].join('::')}${
            theTypeArgsStr ? `<${theTypeArgsStr}>` : ''
        }`;
    }
    return str;
}

interface NormalizedFunctionParameterTypeDetails {
    isTxContext: boolean;
    paramTypeText: string;
}

export function getNormalizedFunctionParameterTypeDetails(
    param: IotaMoveNormalizedType,
    functionTypeArgNames?: string[],
): NormalizedFunctionParameterTypeDetails {
    const paramTypeText = normalizedFunctionParameterTypeToString(param, functionTypeArgNames);
    return {
        isTxContext: paramTypeText === '0x2::tx_context::TxContext',
        paramTypeText,
    };
}
