import api from '@/lib/api';
import useAuthStore, { useIsAuthorized } from '@/lib/store/auth';
import WeatherDashboard from '@/pages/Weather/WeatherDashboard.tsx';
import WeatherSelect from '@/pages/Weather/WeatherSelect.tsx';
import { useEffect, useRef, useState } from 'react';

interface LocationDB {
    description: string;
    is_default: boolean;
    latitude: number;
    longitude: number;
    name: string;
    user_id: number;
}

interface GetLocationDB extends LocationDB {
    created_at: string;
    id: number;
    updated_at: string;
}

export type Location = {
    id: number;
    name: string;
    lat: number;
    lon: number;
};

export type WeatherDashboardProps = {
    nextWindow: () => void;
    locations: Location[];
    setLocations: React.Dispatch<React.SetStateAction<Location[]>>; //(locs: Location[]) => void;
};

function Weather() {
    const [locations, setLocations] = useState<Location[]>([]);

    const prevLocations = useRef<Location[]>([]);
    const [showDashboard, setShowDashboard] = useState(locations.length > 0);
    const [hasMounted, setHasMounted] = useState(false);
    const user = useAuthStore((state) => state.user);

    const isAuthorized = useIsAuthorized();

    useEffect(() => {
        if (isAuthorized) {
            fetchLocations();
        }
    }, [isAuthorized]);

    const fetchLocations = async () => {
        if (!isAuthorized) {
            return;
        }
        setHasMounted(false);

        try {
            const res = await api.get('/weather_locations');

            const data: GetLocationDB[] = res.data;

            const locationsDB: Location[] = data
                .sort((a, b) => (b.is_default ? 1 : 0) - (a.is_default ? 1 : 0))
                .map((loc) => ({
                    id: loc.id,
                    name: loc.name,
                    lat: loc.latitude,
                    lon: loc.longitude,
                }));

            if (locationsDB.length > 0) setShowDashboard(true);

            setLocations(locationsDB);
        } catch (e) {
            console.error('Error fetching locations:', e);
        }
    };

    const createLocation = async (location: Location) => {
        if (!user) {
            return;
        }

        const locationDB: LocationDB = {
            description: '',
            is_default: true,
            latitude: location.lat,
            longitude: location.lon,
            name: location.name,
            user_id: user.id, // ID uživatele, který vytvořil tuto lokaci
        };

        try {
            await api.post('/weather_locations', locationDB, {
                headers: {
                    'Content-Type': 'application/json',
                },
            });

            fetchLocations();
        } catch (e) {
            console.error('Error creating location:', e);
        }
    };

    const editLocation = async (locationDef: Location, location: Location) => {
        if (!user) {
            return;
        }

        try {
            await api.put(`/weather_locations/${locationDef.id}`, {
                description: '',
                is_default: true,
                latitude: locationDef.lat,
                longitude: locationDef.lon,
                name: locationDef.name,
                user_id: user.id, // ID uživatele, který vytvořil tuto lokaci
            });
        } catch (e) {
            console.error('Error editing location:', e);
        }

        try {
            await api.put(`/weather_locations/${location.id}`, {
                description: '',
                is_default: false,
                latitude: location.lat,
                longitude: location.lon,
                name: location.name,
                user_id: user.id, // ID uživatele, který vytvořil tuto lokaci
            });
            fetchLocations();
        } catch (e) {
            console.error('Error editing location:', e);
        }
    };

    const deleteLocation = async (location: Location) => {
        if (!isAuthorized) {
            return;
        }

        try {
            await api.delete(`/weather_locations/${location.id}`);
            fetchLocations();
        } catch (e) {
            console.error('Error deleting location:', e);
        }
    };

    useEffect(() => {
        if (!hasMounted) {
            setHasMounted(true);
            prevLocations.current = locations;
            return;
        }

        if (locations.length > prevLocations.current.length) {
            createLocation(locations[0]);
        } else if (locations.length < prevLocations.current.length) {
            const removedLocation = prevLocations.current.find(
                (prevLoc) => !locations.some((loc) => loc.id === prevLoc.id),
            );

            if (removedLocation) {
                deleteLocation(removedLocation);
            }
        } else {
            if (locations[0] && prevLocations.current[0]) {
                editLocation(locations[0], prevLocations.current[0]);
            }
        }

        prevLocations.current = locations;
    }, [locations]);

    // className="bg-gray-900   w-screen   font-sans"
    return (
        <div className="h-full w-full flex items-center justify-center overflow-y-scroll">
            {showDashboard ? (
                <WeatherDashboard
                    nextWindow={() => setShowDashboard(false)}
                    locations={locations}
                    setLocations={setLocations}
                />
            ) : (
                <WeatherSelect
                    nextWindow={() => setShowDashboard(true)}
                    locations={locations}
                    setLocations={setLocations}
                />
            )}
        </div>
    );
}

export default Weather;
