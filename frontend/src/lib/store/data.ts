import { create } from 'zustand';
import { EarthquakeData } from '../data/getEarthquakes';

interface StoreState {
    earthquake?: EarthquakeData;
    setEarthquake: (earthquake: EarthquakeData) => void;
    clearEarthquake: () => void;
}

const useData = create<StoreState>()((set) => ({
    earthquake: undefined,
    setEarthquake: (earthquake) => set({ earthquake }),
    clearEarthquake: () => set({ earthquake: undefined }),
}));

export default useData;
