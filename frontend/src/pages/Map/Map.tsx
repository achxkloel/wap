import MapComponent from '@/components/Map';
import getEarthquakes, { EarthquakeProperties } from '@/lib/data/getEarthquakes';
import useData from '@/lib/store/data';
import { Feature, Point } from 'geojson';
import { useCallback, useState } from 'react';

function Map() {
    const [location, setLocation] = useState<[number, number]>();
    const earthquakes = useData((state) => state.earthquake);
    const setEarthquakes = useData((state) => state.setEarthquake);

    const handleBoundsChange = useCallback(async (bounds: L.LatLngBounds) => {
        setLocation(undefined);

        try {
            const data = await getEarthquakes({
                maxlatitude: bounds.getNorthEast().lat,
                minlatitude: bounds.getSouthWest().lat,
                maxlongitude: bounds.getNorthEast().lng,
                minlongitude: bounds.getSouthWest().lng,
            });

            setEarthquakes(data);
            console.log('Earthquakes:', data);
        } catch (e) {
            console.error('Error fetching earthquakes:', e);
        }
    }, []);

    const handleEarthquakeClick = (feature: Feature<Point, EarthquakeProperties>) => {
        setLocation([feature.geometry.coordinates[1], feature.geometry.coordinates[0]]);
    };

    return (
        <div className="h-full w-full flex">
            <MapComponent
                location={location}
                locationZoom={10}
                onBoundsChange={handleBoundsChange}
                animate={true}
            />
            <div className="flex flex-col w-[400px] overflow-y-auto">
                {earthquakes &&
                    earthquakes.features.map((feature) => (
                        <div
                            key={feature.id}
                            className="p-4 border-b border-gray-200 hover:bg-gray-50 cursor-pointer"
                            onClick={() => handleEarthquakeClick(feature)}
                        >
                            <h4 className="text-md font-semibold">{feature.properties.place}</h4>
                            <p className="text-sm text-gray-500">Magnitude: {feature.properties.mag}</p>
                        </div>
                    ))}
            </div>
        </div>
    );
}

export default Map;
