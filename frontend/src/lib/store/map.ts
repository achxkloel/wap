import L from 'leaflet';
import { create } from 'zustand';

export type RectangleCoordinates = L.LatLngBounds;
export interface CircleCoordinates {
    center: L.LatLngTuple;
    radius: number;
}

interface MapStoreState {
    draw: boolean;
    mode: 'rectangle' | 'circle';
    coordinates: RectangleCoordinates | CircleCoordinates | null;
    bounds: L.LatLngBounds | null;
    colorStrategy: 'magnitude' | 'significance' | 'date';
    setColorStrategy: (colorStrategy: 'magnitude' | 'significance' | 'date') => void;
    startDraw: (mode: 'rectangle' | 'circle', coordinates?: RectangleCoordinates | CircleCoordinates) => void;
    stopDraw: (coordinates?: RectangleCoordinates | CircleCoordinates) => void;
    setBounds: (bounds: L.LatLngBounds) => void;
}

const useMapStore = create<MapStoreState>()((set) => ({
    draw: false,
    mode: 'rectangle',
    coordinates: null,
    bounds: null,
    colorStrategy: 'magnitude',
    setColorStrategy: (colorStrategy) => set({ colorStrategy }),
    startDraw: (mode, coordinates) => {
        const newState: Partial<MapStoreState> = {
            draw: true,
            mode,
            coordinates: null,
        };

        if (coordinates) {
            newState.coordinates = coordinates;
        }

        set(newState);
    },
    stopDraw: (coordinates) => {
        const newState: Partial<MapStoreState> = {
            draw: false,
        };

        if (coordinates) {
            newState.coordinates = coordinates;
        }

        set(newState);
    },
    setBounds: (bounds) => set({ bounds }),
}));

export default useMapStore;
