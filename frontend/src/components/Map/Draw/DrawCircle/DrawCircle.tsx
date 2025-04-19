import { CircleCoordinates } from '@/lib/store/map';
import L, { LatLngTuple } from 'leaflet';
import { useEffect, useState } from 'react';
import { Circle, useMap, useMapEvents } from 'react-leaflet';

interface DrawCircleProps {
    coordinates?: CircleCoordinates | null;
    onChange: (coordinates: CircleCoordinates | null) => void;
}

function DrawCircle({ coordinates, onChange }: DrawCircleProps) {
    const map = useMap();
    const [center, setCenter] = useState<LatLngTuple | null>(null);
    const [radius, setRadius] = useState<number | null>(null);
    const [drawing, setDrawing] = useState(false);

    useEffect(() => {
        if (coordinates) {
            setCenter(coordinates.center);
            setRadius(coordinates.radius);
            const bounds = L.latLng(coordinates.center).toBounds(coordinates.radius * 2);
            map.fitBounds(bounds);

            if (onChange) {
                onChange({
                    center: coordinates.center,
                    radius: coordinates.radius,
                });
            }
        }
    }, []);

    useMapEvents({
        click(e) {
            if (!drawing) {
                if (center && radius !== null) {
                    const from = L.latLng(center);
                    const to = e.latlng;
                    const distance = from.distanceTo(to);

                    if (distance < radius) {
                        return;
                    }

                    setCenter(null);
                    setRadius(null);

                    if (onChange) {
                        onChange(null);
                    }

                    return;
                }

                setCenter([e.latlng.lat, e.latlng.lng]);
                setRadius(0);
                setDrawing(true);
            } else {
                setDrawing(false);

                if (onChange && center && radius !== null) {
                    onChange({
                        center,
                        radius,
                    });
                }
            }
        },
        mousemove(e) {
            if (drawing && center) {
                const from = L.latLng(center);
                const to = e.latlng;
                const distance = from.distanceTo(to);
                setRadius(distance);
            }
        },
    });

    if (!center || radius === null) {
        return null;
    }

    return (
        <Circle
            center={center}
            radius={radius}
            pathOptions={{
                color: 'red',
                dashArray: '8, 8',
                weight: 2,
            }}
        />
    );
}

export default DrawCircle;
