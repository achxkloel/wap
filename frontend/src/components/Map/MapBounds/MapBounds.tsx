import useMapStore from '@/lib/store/map';
import 'leaflet/dist/leaflet.css';
import { useEffect } from 'react';
import { useMap } from 'react-leaflet';

function MapBounds() {
    const map = useMap();
    const setBounds = useMapStore((state) => state.setBounds);

    useEffect(() => {
        if (!map) return;

        const initialBounds = map.getBounds();
        setBounds(initialBounds);

        const handleMoveEnd = () => {
            const bounds = map.getBounds();
            setBounds(bounds);
        };

        map.on('moveend', handleMoveEnd);

        return () => {
            map.off('moveend', handleMoveEnd);
        };
    }, [map]);

    return null;
}

export default MapBounds;
