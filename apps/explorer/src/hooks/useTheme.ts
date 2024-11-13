// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { useContext } from 'react';
import { ThemeContext } from '~/contexts';

export function useTheme() {
    if (!ThemeContext) {
        throw new Error('useTheme must be used within a ThemeProvider');
    }

    return useContext(ThemeContext);
}
