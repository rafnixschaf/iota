// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React, { createContext, useContext, useState, useEffect, PropsWithChildren } from 'react';

export enum Theme {
    Light = 'light',
    Dark = 'dark',
}

interface ThemeContextType {
    theme: Theme;
    setTheme: (theme: Theme) => void;
}

export const ThemeContext = createContext<ThemeContextType>({
    theme: Theme.Light,
    setTheme: () => {},
});

interface ThemeProviderProps {
    appId: string;
}

export function ThemeProvider({ children, appId }: PropsWithChildren<ThemeProviderProps>) {
    const storageKey = `theme_${appId}`;
    const [theme, setTheme] = useState<Theme>(() => {
        if (typeof window !== 'undefined' && localStorage.getItem(storageKey)) {
            return localStorage.getItem(storageKey) as Theme;
        }
        return Theme.Light;
    });

    useEffect(() => {
        if (typeof window !== 'undefined') {
            localStorage.setItem(storageKey, theme);
        }
        document.documentElement.classList.toggle(Theme.Dark, theme === Theme.Dark);
    }, [theme, storageKey]);

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

export const useTheme = (): ThemeContextType => {
    const context = useContext(ThemeContext);
    if (!context) {
        throw new Error('useTheme must be used within a ThemeProvider');
    }
    return context;
};
