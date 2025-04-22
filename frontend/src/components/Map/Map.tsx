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
    animate?: boolean;
}

function Map({ location, locationZoom, animate }: MapProps) {
    return (
        <MapContainer
            center={[51.505, -0.09]}
            zoom={13}
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
            />
            <Draw />
            <MapLayers />
            <Legend />
        </MapContainer>
    );
}

export default Map;
