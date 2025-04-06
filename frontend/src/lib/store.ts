import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface StoreState {
    theme: 'light' | 'dark';
    counter: number;
    toggleTheme: () => void;
    increment: () => void;
    decrement: () => void;
    reset: () => void;
}

const useStore = create<StoreState>()(
    persist(
        (set) => ({
            theme: 'light',
            counter: 0,
            toggleTheme: () => set((state) => ({ theme: state.theme === 'light' ? 'dark' : 'light' })),
            increment: () => set((state) => ({ counter: state.counter + 1 })),
            decrement: () => set((state) => ({ counter: state.counter - 1 })),
            reset: () => set({ counter: 0 }),
        }),
        {
            name: 'client-settings',
            partialize: (state) => ({ theme: state.theme }),
        },
    ),
);

export default useStore;
