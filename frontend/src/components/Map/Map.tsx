import useData from '@/lib/store/data';
import L from 'leaflet';
import 'leaflet/dist/leaflet.css';
import { CircleMarker, FeatureGroup, LayersControl, MapContainer, TileLayer } from 'react-leaflet';
import MapBounds from './MapBounds';
import MapLocation from './MapLocation';

interface MapProps {
    location?: L.LatLngExpression;
    locationZoom?: number;
    animate?: boolean;
    onBoundsChange?: (bounds: L.LatLngBounds) => void;
}

function Map({ location, locationZoom, animate, onBoundsChange }: MapProps) {
    const earthquakes = useData((state) => state.earthquake);

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

    const renderLayers = () => {
        return <LayersControl position="topright">{renderEarthquakeLayer()}</LayersControl>;
    };

    return (
        <MapContainer
            center={[51.505, -0.09]}
            zoom={13}
            className="h-full w-full"
        >
            <TileLayer
                attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
            />
            <MapBounds onBoundsChange={onBoundsChange} />
            <MapLocation
                location={location}
                zoom={locationZoom}
                animate={animate}
            />
            {renderLayers()}
        </MapContainer>
    );
}

export default Map;
