import 'leaflet/dist/leaflet.css';
import { useEffect } from 'react';
import { useMap } from 'react-leaflet';

interface MapLocationProps {
    location?: L.LatLngExpression;
    zoom?: number;
    animate?: boolean;
}

function MapLocation({ location, zoom, animate = true }: MapLocationProps) {
    const map = useMap();

    useEffect(() => {
        if (!location) {
            return;
        }

        const locationZoom = zoom || map.getZoom();

        if (animate) {
            map.flyTo(location, locationZoom);
        } else {
            map.setView(location, locationZoom);
        }
    }, [map, location, zoom]);

    return null;
}

export default MapLocation;
