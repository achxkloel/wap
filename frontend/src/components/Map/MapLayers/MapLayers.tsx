import { EarthquakeProperties } from '@/lib/data/earthquakes/types';
import useData from '@/lib/store/data';
import useMapStore from '@/lib/store/map';
import { Feature, Point } from 'geojson';
import L from 'leaflet';
import { useEffect } from 'react';
import { CircleMarker, FeatureGroup, LayersControl, useMap } from 'react-leaflet';
import { getDateColor, getMagnitudeColor, getSignificanceColor, getSize } from '../Legend/Legend';

function MapLayers() {
    const map = useMap();
    const earthquakes = useData((state) => state.earthquake);
    const colorStrategy = useMapStore((state) => state.colorStrategy);
    const selected = useData((state) => state.selected);
    const setSelected = useData((state) => state.setSelected);

    useEffect(() => {
        if (typeof selected === 'undefined') {
            return;
        }

        if (!earthquakes) {
            return;
        }

        const earthquake = earthquakes?.features.find((feature) => feature.id === selected);

        if (!earthquake) {
            return;
        }

        const lat = earthquake.geometry.coordinates[1];
        const lng = earthquake.geometry.coordinates[0];
        map.setView([lat, lng], map.getZoom());
    }, [selected]);

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

    const getFillColor = (feature: Feature<Point, EarthquakeProperties>) => {
        let fn;
        let param;

        switch (colorStrategy) {
            case 'magnitude':
                fn = getMagnitudeColor;
                param = feature.properties.mag;
                break;
            case 'significance':
                fn = getSignificanceColor;
                param = feature.properties.sig;
                break;
            case 'date':
                fn = getDateColor;
                param = feature.properties.time;
                break;
            default:
                fn = getMagnitudeColor;
                param = feature.properties.mag;
        }

        return fn(param).color;
    };

    const getRadius = (feature: Feature<Point, EarthquakeProperties>) => {
        return getSize(feature.properties.mag).size / 2;
    };

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
                    {[...earthquakes.features]
                        .sort((a, b) => (a.id === selected ? 1 : b.id === selected ? -1 : 0))
                        .map((feature) => {
                            return (
                                <CircleMarker
                                    key={`${feature.id}-${colorStrategy}-${selected}`}
                                    center={[feature.geometry.coordinates[1], feature.geometry.coordinates[0]]}
                                    stroke={true}
                                    color="#000000"
                                    weight={1}
                                    fillColor={selected === feature.id ? '#dbeafe' : getFillColor(feature)}
                                    fillOpacity={1}
                                    radius={getRadius(feature)}
                                    eventHandlers={{
                                        click: () => {
                                            setSelected(feature.id);
                                        },
                                    }}
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
