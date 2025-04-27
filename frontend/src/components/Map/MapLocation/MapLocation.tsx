import 'leaflet/dist/leaflet.css';
import React, { useEffect } from 'react';
import { Circle, Marker, useMap } from 'react-leaflet';

interface MapLocationProps {
    location?: L.LatLngExpression;
    zoom?: number;
    radius?: number;
    animate?: boolean;
}

function MapLocation({ location, zoom, radius = 50, animate = true }: MapLocationProps) {
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

    if (!location) {
        return null;
    }

    return (
        <React.Fragment>
            <Marker position={location} />
            {typeof radius === 'number' && (
                <Circle
                    center={location}
                    radius={radius * 1000}
                />
            )}
        </React.Fragment>
    );
}

export default MapLocation;
