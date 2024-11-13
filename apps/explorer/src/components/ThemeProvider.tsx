// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { useState, type PropsWithChildren } from 'react';
import { ThemeContext } from '~/contexts';
import { Theme } from '~/lib/ui';

export function ThemeProvider({ children }: PropsWithChildren): React.JSX.Element {
    const [theme, setTheme] = useState<Theme>(Theme.Light);

    return (
        <ThemeContext.Provider
            value={{
                theme,
                setTheme,
            }}
        >
            {children}
        </ThemeContext.Provider>
    );
}
