import 'leaflet/dist/leaflet.css';
import { useEffect } from 'react';
import { useMap } from 'react-leaflet';

interface MapBoundsProps {
    onBoundsChange?: (bounds: L.LatLngBounds) => void;
}

function MapBounds({ onBoundsChange }: MapBoundsProps) {
    const map = useMap();

    useEffect(() => {
        if (!map) return;

        const initialBounds = map.getBounds();

        if (onBoundsChange) {
            onBoundsChange(initialBounds);
        }

        const handleMoveEnd = () => {
            const bounds = map.getBounds();

            if (onBoundsChange) {
                onBoundsChange(bounds);
            }
        };

        map.on('moveend', handleMoveEnd);

        return () => {
            map.off('moveend', handleMoveEnd);
        };
    }, [map, onBoundsChange]);

    return null;
}

export default MapBounds;
