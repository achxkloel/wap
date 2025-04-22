import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface UserInfo {
    id: number;
    email: string;
    firstName?: string | null;
    lastName?: string | null;
    imageUrl?: string | null;
    provider?: string | null;
    googleId?: string | null;
    createdAt: string;
    updatedAt: string;
}

interface StoreState {
    user: UserInfo | null;
    access_token: string | null;
    refresh_token: string | null;
    setUser: (user: UserInfo | null) => void;
    setAccessToken: (token: string) => void;
    setRefreshToken: (token: string) => void;
    removeAccessToken: () => void;
    removeRefreshToken: () => void;
}

const useAuthStore = create<StoreState>()(
    persist(
        (set) => ({
            user: null,
            access_token: null,
            refresh_token: null,
            /* eslint-disable @typescript-eslint/no-explicit-any */
            setUser: (user: Record<string, any> | null) => {
                if (user === null) {
                    set({ user });
                    return;
                }

                set({
                    user: {
                        id: user.id,
                        email: user.email,
                        firstName: user.first_name,
                        lastName: user.last_name,
                        imageUrl: user.image_url,
                        provider: user.provider,
                        googleId: user.google_id,
                        createdAt: user.created_at,
                        updatedAt: user.updated_at,
                    },
                });
            },
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
