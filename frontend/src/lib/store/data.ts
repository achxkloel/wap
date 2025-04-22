import { create } from 'zustand';
import { EarthquakeData } from '../data/earthquakes/types';

interface StoreState {
    earthquake?: EarthquakeData;
    selected?: string | number;
    setSelected: (selected: string | number | undefined) => void;
    setEarthquake: (earthquake: EarthquakeData) => void;
    clearEarthquake: () => void;
}

const useData = create<StoreState>()((set) => ({
    earthquake: undefined,
    selected: undefined,
    setSelected: (selected) =>
        set((state) => {
            if (state.selected === selected) {
                return { selected: undefined };
            }
            return { selected };
        }),
    setEarthquake: (earthquake) => set({ earthquake, selected: undefined }),
    clearEarthquake: () => set({ earthquake: undefined }),
}));

export default useData;
