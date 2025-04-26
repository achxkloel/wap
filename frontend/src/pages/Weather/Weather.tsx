import api from '@/lib/api';
import { logger } from '@/lib/logger';
import { useIsAuthorized } from '@/lib/store/auth';
import WeatherDashboard from '@/pages/Weather/WeatherDashboard.tsx';
import WeatherSelect from '@/pages/Weather/WeatherSelect.tsx';
import { useEffect, useState } from 'react';

export type Location = {
    name: string;
    lat: number;
    lon: number;
};

export type WeatherDashboardProps = {
    nextWindow: () => void;
    locations: Location[];
    setLocations: (locs: Location[]) => void;
};

function Weather() {
    const [locations, setLocations] = useState<Location[]>([]);
    const [showDashboard, setShowDashboard] = useState(locations.length > 0);
    const isAuthorized = useIsAuthorized();

    useEffect(() => {
        if (!isAuthorized) {
            return;
        }

        fetchLocations();
    }, []);

    const fetchLocations = async () => {
        try {
            logger.debug('Fetching locations...');
            const res = await api.get('/weather_locations');
            setLocations(res.data); // TODO map

            if (locations.length > 0) setShowDashboard(true);
        } catch (error) {
            logger.error('Error fetching locations:', error);
        }
    };

    const setLocationsAndDB = (locs: Location[]) => {
        setLocations(locs);
        // TODO set db
    };

    // const api = axios.create({
    //     baseURL: 'https://api.open-meteo.com/v1',
    // });

    // function get(lat: number, lon: number) {
    //     return api
    //         .get('/forecast', {
    //             params: {
    //                 latitude: lat,
    //                 longitude: lon,
    //                 current_weather: true,
    //                 hourly: 'temperature_2m,precipitation,relative_humidity_2m,windspeed_10m,winddirection_10m,uv_index,surface_pressure,cloudcover',
    //                 daily: 'temperature_2m_max,temperature_2m_min,precipitation_sum',
    //                 timezone: 'Europe/Prague',
    //             },
    //         })
    //         .then((res) => res.data);
    // }

    // import get from '@/lib/data/weather/get.ts';

    // try {
    //     get();
    // } catch (error) {}

    // className="bg-gray-900   w-screen   font-sans"
    return (
        <div className="h-full w-full flex items-center justify-center overflow-y-scroll">
            {showDashboard ? (
                <WeatherDashboard
                    nextWindow={() => setShowDashboard(false)}
                    locations={locations}
                    setLocations={setLocationsAndDB}
                />
            ) : (
                <WeatherSelect
                    nextWindow={() => setShowDashboard(true)}
                    locations={locations}
                    setLocations={setLocationsAndDB}
                />
            )}
        </div>
    );
}

export default Weather;
