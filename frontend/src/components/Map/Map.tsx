import L from 'leaflet';
import 'leaflet/dist/leaflet.css';
import { MapContainer, TileLayer } from 'react-leaflet';
import Draw from './Draw';
import Legend from './Legend';
import MapBounds from './MapBounds';
import MapLayers from './MapLayers';
import MapLocation from './MapLocation';

interface MapProps {
    location?: L.LatLngExpression;
    locationZoom?: number;
    locationRadius?: number;
    animate?: boolean;
    showLayers?: boolean;
    showLegend?: boolean;
    showDraw?: boolean;
}

function Map({
    location,
    locationZoom,
    locationRadius,
    animate,
    showLayers = false,
    showLegend = false,
    showDraw = false,
}: MapProps) {
    return (
        <MapContainer
            center={[0, 0]}
            zoom={3}
            className="h-full w-full"
            preferCanvas={true}
        >
            <TileLayer
                attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                className="map-tiles"
            />
            <MapBounds />
            <MapLocation
                location={location}
                zoom={locationZoom}
                animate={animate}
                radius={locationRadius}
            />
            {showDraw && <Draw />}
            {showLayers && <MapLayers />}
            {showLegend && <Legend />}
        </MapContainer>
    );
}

export default Map;
