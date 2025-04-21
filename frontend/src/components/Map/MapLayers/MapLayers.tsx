import useData from '@/lib/store/data';
import L from 'leaflet';
import { useEffect } from 'react';
import { CircleMarker, FeatureGroup, LayersControl, useMap } from 'react-leaflet';

function MapLayers() {
    const map = useMap();
    const earthquakes = useData((state) => state.earthquake);

    useEffect(() => {
        if (earthquakes && earthquakes.bbox) {
            let bounds = L.latLngBounds(
                [earthquakes.bbox[1], earthquakes.bbox[0]],
                [earthquakes.bbox[3], earthquakes.bbox[2]],
            );

            if (earthquakes.bbox.length === 6) {
                bounds = L.latLngBounds(
                    [earthquakes.bbox[1], earthquakes.bbox[0]],
                    [earthquakes.bbox[4], earthquakes.bbox[3]],
                );
            }

            map.fitBounds(bounds);
        }
    }, [earthquakes]);

    const renderEarthquakeLayer = () => {
        if (!earthquakes) {
            return null;
        }

        const getColor = (value: number, lowLimit: number, highLimit: number) => {
            const normalizedMagnitude = Math.min(Math.max(value, lowLimit), highLimit);
            const normalizedZeroToOne = normalizedMagnitude / highLimit;

            const startColor = [99, 159, 255];
            const endColor = [255, 66, 69];

            const color = startColor.map((start, index) => {
                return Math.round(start + (endColor[index] - start) * normalizedZeroToOne);
            });
            return `rgb(${color.join(',')})`;
        };

        return (
            <LayersControl.Overlay
                name="Earthquakes"
                checked
            >
                <FeatureGroup>
                    {earthquakes.features.map((feature) => (
                        <CircleMarker
                            key={feature.id}
                            center={[feature.geometry.coordinates[1], feature.geometry.coordinates[0]]}
                            color={getColor(feature.properties.sig, 0, 1000)}
                            radius={Math.max(10, feature.properties.mag * 2)}
                        />
                    ))}
                </FeatureGroup>
            </LayersControl.Overlay>
        );
    };

    return <LayersControl position="topright">{renderEarthquakeLayer()}</LayersControl>;
}

export default MapLayers;
