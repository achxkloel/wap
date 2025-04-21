import useData from '@/lib/store/data';
import useMapStore from '@/lib/store/map';
import L from 'leaflet';
import { useEffect } from 'react';
import { CircleMarker, FeatureGroup, LayersControl, useMap } from 'react-leaflet';
import { dateColors, magnitudeColors, magnitudeSizes, significanceColors } from '../Legend/Legend';

function MapLayers() {
    const map = useMap();
    const earthquakes = useData((state) => state.earthquake);
    const colorStrategy = useMapStore((state) => state.colorStrategy);

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

        return (
            <LayersControl.Overlay
                name="Earthquakes"
                checked
            >
                <FeatureGroup>
                    {earthquakes.features.map((feature) => {
                        const getFillColor = () => {
                            let colors;
                            let param;

                            switch (colorStrategy) {
                                case 'magnitude':
                                    colors = magnitudeColors;
                                    param = feature.properties.mag;
                                    break;
                                case 'significance':
                                    colors = significanceColors;
                                    param = feature.properties.sig;
                                    break;
                                case 'date':
                                    colors = dateColors;
                                    param = feature.properties.time;
                                    break;
                                default:
                                    colors = magnitudeColors;
                                    param = feature.properties.mag;
                            }

                            return colors.find((color) => color.cond(param))?.color || '#000000';
                        };

                        const getRadius = () => {
                            const size = magnitudeSizes.find((size) => size.cond(feature.properties.mag))?.size;
                            return size ? size / 2 : 20;
                        };

                        return (
                            <CircleMarker
                                key={`${feature.id}-${colorStrategy}`}
                                center={[feature.geometry.coordinates[1], feature.geometry.coordinates[0]]}
                                stroke={true}
                                color="#000000"
                                weight={1}
                                fillColor={getFillColor()}
                                fillOpacity={1}
                                radius={getRadius()}
                            />
                        );
                    })}
                </FeatureGroup>
            </LayersControl.Overlay>
        );
    };

    return <LayersControl position="topright">{renderEarthquakeLayer()}</LayersControl>;
}

export default MapLayers;
