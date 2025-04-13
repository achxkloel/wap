import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface StoreState {
    token: string | null;
    setToken: (token: string) => void;
    removeToken: () => void;
}

const useAuthStore = create<StoreState>()(
    persist(
        (set) => ({
            token: null,
            setToken: (token: string) => set({ token }),
            removeToken: () => set({ token: null }),
        }),
        {
            name: 'auth-token',
            partialize: (state) => ({ token: state.token }),
        },
    ),
);

export default useAuthStore;
