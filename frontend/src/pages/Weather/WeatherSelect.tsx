import React, { useEffect, useState } from 'react';
import type { WeatherDashboardProps, Location } from '@/pages/Weather/Weather.tsx';

const allCities = [
    'London',
    'Paris',
    'Berlin',
    'Madrid',
    'Rome',
    'Istanbul',
    'Warsaw',
    'Vienna',
    'Bucharest',
    'Prague',
    'Tokyo',
    'Seoul',
    'Shanghai',
    'Beijing',
    'Bangkok',
    'Mumbai',
    'Jakarta',
    'Manila',
    'Cairo',
    'Lagos',
    'Nairobi',
    'Johannesburg',
    'Sydney',
    'Melbourne',
    'Auckland',
];

type CityCardProps = {
    name: string;
    setLocations: React.Dispatch<React.SetStateAction<Location[]>>;
    nextWindow: () => void;
};

type CitySearchResult = {
    name: string;
    latitude: number;
    longitude: number;
    country: string;
};

type City = {
    name: string;
    latitude: number;
    longitude: number;
};

function CityCard({ name, setLocations, nextWindow }: CityCardProps) {
    const [image, setImage] = useState<string>('');
    const [temp, setTemp] = useState<string>('?');
    const [rain, setRain] = useState<string>('?');
    const [wind, setWind] = useState<string>('?');
    const [lat, setLat] = useState<number | null>(null);
    const [lon, setLon] = useState<number | null>(null);

    useEffect(() => {
        const fetchData = async () => {
            const geoRes = await fetch(`https://geocoding-api.open-meteo.com/v1/search?name=${name}&count=1`);
            const geoData = await geoRes.json();
            const latitude = geoData.results?.[0]?.latitude;
            const longitude = geoData.results?.[0]?.longitude;

            if (latitude && longitude) {
                setLat(latitude);
                setLon(longitude);

                const weatherRes = await fetch(
                    `https://api.open-meteo.com/v1/forecast?latitude=${latitude}&longitude=${longitude}&current_weather=true&daily=precipitation_sum&timezone=Europe%2FPrague`,
                );
                const weatherData = await weatherRes.json();
                const current = weatherData.current_weather;
                setTemp(current.temperature.toFixed(1));
                setWind(current.windspeed.toFixed(0));

                const dailyRain = weatherData.daily?.precipitation_sum?.[0];
                setRain(dailyRain ? dailyRain.toFixed(1) : '0');
            }

            try {
                const unsplashKey = 'QomxOmr0uAe3rY0cL076U6MDaOCaBWfrd0DhQjmQCIo';
                const response = await fetch(
                    `https://api.unsplash.com/search/photos?query=${encodeURIComponent(name)}&client_id=${unsplashKey}`,
                );

                if (!response.ok) {
                    throw new Error(`HTTP error: ${response.status}`);
                }

                const imgData = await response.json();
                const imgUrl = imgData.results?.[0]?.urls?.regular;

                setImage(imgUrl || 'https://cdn-icons-png.flaticon.com/512/69/69524.png');
            } catch {
                setImage('https://cdn-icons-png.flaticon.com/512/69/69524.png');
            }
        };

        fetchData();
    }, [name]);

    const handleClick = () => {
        if (lat !== null && lon !== null) {
            setLocations((prev) => [{ name, lat, lon }, ...prev]);
            nextWindow();
        }
    };

    return (
        <div
            onClick={handleClick}
            className="bg-sidebar-border p-4 flex flex-col items-center  rounded-lg shadow-xl hover:scale-105 transform transition-all duration-300 ease-in-out cursor-pointer"
        >
            <img
                src={image}
                alt={`City ${name}`}
                className="w-[300vw] h-[300vw] max-w-[200px] max-h-[200px] flex-shrink-0 rounded-xl object-cover shadow-md mb-4"
            />
            <div className="text-sm mt-2 text-center">
                <p className="text-lg font-bold">{name}</p>
                <p className="text-red-800">
                    Temperature: <span className="font-semibold">{temp}°C</span>
                </p>
                <p className="text-blue-800">
                    Precipitation: <span className="font-semibold">{rain} mm</span>
                </p>
                <p className="text-green-800">
                    Wind: <span className="font-semibold">{wind} km/h</span>
                </p>
            </div>
        </div>
    );
}

function CitySearch({ nextWindow, setLocations }: WeatherDashboardProps) {
    const [query, setQuery] = useState('');
    const [results, setResults] = useState<CitySearchResult[]>([]);
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        const timeout = setTimeout(() => {
            if (query.length >= 2) {
                setLoading(true);
                fetch(`https://geocoding-api.open-meteo.com/v1/search?name=${query}&count=5`)
                    .then((res) => res.json())
                    .then((data) => {
                        setResults(data.results || []);
                        setLoading(false);
                    })
                    .catch(() => setLoading(false));
            } else {
                setResults([]);
            }
        }, 400);

        return () => clearTimeout(timeout);
    }, [query]);

    const handleSelect = (city: City) => {
        const location = {
            name: city.name,
            lat: city.latitude,
            lon: city.longitude,
        };
        setLocations((prev: Location[]) => [location, ...prev]);
        nextWindow();
    };

    return (
        <div className="w-[395px] mx-auto">
            <input
                type="text"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search for a city..."
                className="w-full p-3 border-2 text-sidebar-foreground rounded-xl text-lg shadow-md mb-4 bg-sidebar-border"
            />
            {loading && <div className="text-gray-500">Searching...</div>}
            <div className="space-y-2">
                {results.map((city, index) => (
                    <div
                        key={index}
                        className="p-4 text-sm bg-sidebar-border rounded-lg shadow hover:bg-gray-200 cursor-pointer transition"
                        onClick={() => handleSelect(city)}
                    >
                        <div className="font-semibold text-lg">{city.name}</div>
                        {city.country} • lat: {city.latitude}, lon: {city.longitude}
                    </div>
                ))}
            </div>
        </div>
    );
}

function WeatherSelect({ nextWindow, locations, setLocations }: WeatherDashboardProps) {
    const randomCities = allCities.sort(() => 0.5 - Math.random()).slice(0, 4);

    return (
        <div className="grid grid-cols-3 gap-8">
            {/* 4 rohové karty */}
            {randomCities.map((city, index) => (
                <div
                    key={index}
                    className={`col-start-${index % 2 === 0 ? 1 : 3} row-start-${Math.floor(index / 2) + 1} flex justify-center items-center `}
                >
                    <CityCard
                        name={city}
                        setLocations={setLocations}
                        nextWindow={nextWindow}
                    />
                </div>
            ))}

            {/* Střed */}
            <div className=" col-start-2 row-start-1  row-span-2 flex flex-col">
                {locations.length === 0 ? (
                    <div className="text-xl font-semibold mb-2">No location selected</div>
                ) : (
                    <div></div>
                )}
                <br />
                <CitySearch
                    setLocations={setLocations}
                    locations={locations}
                    nextWindow={nextWindow}
                />
            </div>
        </div>
    );
}

export default WeatherSelect;
