import { create } from 'zustand';
import { EarthquakeData } from '../data/earthquakes/types';

interface EarthquakeStoreState {
    earthquakes?: EarthquakeData;
    selected?: string | number;
    setSelected: (selected: string | number | undefined) => void;
    setEarthquakes: (earthquakes: EarthquakeData) => void;
    clearEarthquakes: () => void;
}

const useEarthquakeStore = create<EarthquakeStoreState>()((set) => ({
    earthquakes: undefined,
    selected: undefined,
    setSelected: (selected) =>
        set((state) => {
            if (state.selected === selected) {
                return { selected: undefined };
            }
            return { selected };
        }),
    setEarthquakes: (earthquakes) => set({ earthquakes, selected: undefined }),
    clearEarthquakes: () => set({ earthquakes: undefined }),
}));

export default useEarthquakeStore;
