import { RectangleCoordinates } from '@/lib/store/map';
import L, { latLngBounds, LatLngTuple } from 'leaflet';
import { useEffect, useState } from 'react';
import { Rectangle, useMap, useMapEvents } from 'react-leaflet';

interface DrawRectangleProps {
    coordinates?: RectangleCoordinates | null;
    onChange: (coordinates: RectangleCoordinates | null) => void;
}

function DrawRectangle({ coordinates, onChange }: DrawRectangleProps) {
    const map = useMap();
    const [start, setStart] = useState<LatLngTuple | null>(null);
    const [end, setEnd] = useState<LatLngTuple | null>(null);
    const [drawing, setDrawing] = useState(false);
    const bounds = start && end ? latLngBounds([start, end]) : null;

    useEffect(() => {
        if (coordinates) {
            setStart([coordinates.getSouthWest().lat, coordinates.getSouthWest().lng]);
            setEnd([coordinates.getNorthEast().lat, coordinates.getNorthEast().lng]);
            map.fitBounds(coordinates);

            if (onChange) {
                onChange(coordinates);
            }
        }
    }, []);

    useMapEvents({
        click(e) {
            if (!drawing) {
                if (bounds && bounds.contains([e.latlng.lat, e.latlng.lng])) {
                    return;
                }

                if (bounds) {
                    setStart(null);
                    setEnd(null);

                    if (onChange) {
                        onChange(null);
                    }

                    return;
                }

                setStart([e.latlng.lat, e.latlng.lng]);
                setEnd(null);
                setDrawing(true);
            } else {
                setEnd([e.latlng.lat, e.latlng.lng]);
                setDrawing(false);

                if (onChange && start && end) {
                    onChange(L.latLngBounds([start, end]));
                }
            }
        },
        mousemove(e) {
            if (!drawing) {
                return;
            }

            setEnd([e.latlng.lat, e.latlng.lng]);
        },
    });

    if (!bounds) {
        return null;
    }

    return (
        <Rectangle
            bounds={bounds}
            pathOptions={{
                color: 'red',
                dashArray: '8, 8',
                weight: 2,
            }}
        />
    );
}

export default DrawRectangle;
