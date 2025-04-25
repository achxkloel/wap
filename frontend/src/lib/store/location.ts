import { create } from 'zustand';

interface LocationStoreState {
    locations?: any;
    setLocations: (earthquake: any) => void;
    clearLocations: () => void;
}

const useLocationStore = create<LocationStoreState>()((set) => ({
    locations: undefined,
    setLocations: (locations) => set({ locations }),
    clearLocations: () => set({ locations: undefined }),
}));

export default useLocationStore;
