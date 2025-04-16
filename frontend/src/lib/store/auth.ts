import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface StoreState {
    access_token: string | null;
    refresh_token: string | null;
    setAccessToken: (token: string) => void;
    setRefreshToken: (token: string) => void;
    removeAccessToken: () => void;
    removeRefreshToken: () => void;
}

const useAuthStore = create<StoreState>()(
    persist(
        (set) => ({
            access_token: null,
            refresh_token: null,
            setAccessToken: (token: string) => set({ access_token: token }),
            setRefreshToken: (token: string) => set({ refresh_token: token }),
            removeAccessToken: () => set({ access_token: null }),
            removeRefreshToken: () => set({ refresh_token: null }),
        }),
        {
            name: 'auth-token',
            partialize: (state) => ({ access_token: state.access_token, refresh_token: state.refresh_token }),
        },
    ),
);

export const useIsAuthorized = () => useAuthStore((state) => state.access_token !== null);
export const isAuthorized = () => useAuthStore.getState().access_token !== null;

export default useAuthStore;
