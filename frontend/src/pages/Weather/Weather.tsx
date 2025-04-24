import api from '@/lib/api';
import WeatherDashboard from '@/pages/Weather/WeatherDashboard.tsx';
import WeatherSelect from '@/pages/Weather/WeatherSelect.tsx';
import { logger } from '@/lib/logger';
import { useIsAuthorized } from '@/lib/store/auth';
import React, { useEffect, useState } from 'react';

export type Location = {
    name: string;
    lat: number;
    lon: number;
};

export type WeatherDashboardProps = {
    nextWindow: () => void;
    locations: Location[];
    setLocations: React.Dispatch<React.SetStateAction<Location[]>>;
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
            setLocations(res.data);
        } catch (error) {
            logger.error('Error fetching locations:', error);
        }
    };

    // const api = axios .create({
    //     baseURL: 'https://api.open-meteo.com/v1',
    // });

    // function get(lat: number, lon: number) {
    //      return api.get('/forecast', {
    //             params: {
    //                 latitude: lat,
    //                 longitude: lon,
    //                 current_weather: true,
    //                 hourly: 'temperature_2m,precipitation,relative_humidity_2m,windspeed_10m,winddirection_10m,uv_index,surface_pressure,cloudcover',
    //                 daily: 'temperature_2m_max,temperature_2m_min,precipitation_sum',
    //                 timezone: 'Europe/Prague',
    //             },
    //         }).then((res) => res.data);
    // }

    // try {
    //     get()
    // } catch (error) {

    // }

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
