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
    maxRadius: number | null;
    bounds: L.LatLngBounds | null;
    colorStrategy: 'magnitude' | 'significance' | 'date';
    setColorStrategy: (colorStrategy: 'magnitude' | 'significance' | 'date') => void;
    startDraw: (
        mode: 'rectangle' | 'circle',
        opts?: { coordinates?: RectangleCoordinates | CircleCoordinates; maxRadius?: number },
    ) => void;
    stopDraw: (coordinates?: RectangleCoordinates | CircleCoordinates) => void;
    setBounds: (bounds: L.LatLngBounds) => void;
}

const useMapStore = create<MapStoreState>()((set) => ({
    draw: false,
    mode: 'rectangle',
    coordinates: null,
    maxRadius: null,
    bounds: null,
    colorStrategy: 'magnitude',
    setColorStrategy: (colorStrategy) => set({ colorStrategy }),
    startDraw: (mode, opts) => {
        const newState: Partial<MapStoreState> = {
            draw: true,
            mode,
            coordinates: null,
            maxRadius: null,
        };

        if (opts && opts.coordinates) {
            newState.coordinates = opts.coordinates;
        }

        if (opts && opts.maxRadius) {
            newState.maxRadius = opts.maxRadius;
        }

        set(newState);
    },
    stopDraw: (coordinates) => {
        const newState: Partial<MapStoreState> = {
            draw: false,
            maxRadius: null,
        };

        if (coordinates) {
            newState.coordinates = coordinates;
        }

        set(newState);
    },
    setBounds: (bounds) => set({ bounds }),
}));

export default useMapStore;
