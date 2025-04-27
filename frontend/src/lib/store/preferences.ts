import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export type Theme = 'light' | 'dark' | 'system';

interface PreferencesStoreState {
    theme: Theme;
    setTheme: (theme: Theme) => void;
}

const usePreferences = create<PreferencesStoreState>()(
    persist(
        (set) => ({
            theme: 'system',
            setTheme: (theme: 'light' | 'dark' | 'system') => set({ theme }),
        }),
        {
            name: 'preferences',
            partialize: (state) => ({ theme: state.theme }),
        },
    ),
);

export default usePreferences;
