import api from '@/lib/api';
import { logger } from '@/lib/logger';
import { useIsAuthorized } from '@/lib/store/auth';
import React, { useEffect, useState } from 'react';

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
            className="p-4 flex flex-col items-center bg-gray-200 rounded-lg shadow-xl hover:scale-105 transform transition-all duration-300 ease-in-out cursor-pointer"
        >
            <img
                src={image}
                alt={`City ${name}`}
                className="w-[300vw] h-[300vw] max-w-[200px] max-h-[200px] flex-shrink-0 rounded-xl object-cover shadow-md mb-4"
            />
            <div className="text-sm mt-2 text-center">
                <p className="text-lg font-bold">{name}</p>
                <p className="text-gray-800">
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

type CitySearchResult = {
    name: string;
    latitude: number;
    longitude: number;
    country: string;
};

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

    interface City {
        name: string;
        latitude: number;
        longitude: number;
    }

    return (
        <div className="w-[395px] mx-auto">
            <input
                type="text"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search for a city..."
                className="w-full p-3 border-2 border-gray-700 rounded-xl text-lg shadow-md mb-4"
            />
            {loading && <div className="text-gray-600">Searching...</div>}
            <div className="space-y-2">
                {results.map((city, index) => (
                    <div
                        key={index}
                        className="p-4 bg-gray-100 rounded-lg shadow hover:bg-gray-200 cursor-pointer transition"
                        onClick={() => handleSelect(city)}
                    >
                        <div className="font-semibold text-lg">{city.name}</div>
                        <div className="text-sm text-gray-600">
                            {city.country} • lat: {city.latitude}, lon: {city.longitude}
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}

function WeatherSelect({ nextWindow, locations, setLocations }: WeatherDashboardProps) {
    const randomCities = allCities.sort(() => 0.5 - Math.random()).slice(0, 4);

    return (
        <div className="flex justify-center items-center min-h-screen">
            <div className="grid grid-cols-3 gap-8">
                {/* 4 rohové karty */}
                {randomCities.map((city, index) => (
                    <div
                        key={index}
                        className={`col-start-${index % 2 === 0 ? 1 : 3} row-start-${Math.floor(index / 2) + 1} flex justify-center items-center`}
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
                        <div className="text-xl font-semibold mb-2 text-gray-800">No location selected</div>
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
        </div>
    );
}

type WeatherData = {
    current_weather: {
        temperature: number;
        windspeed: number;
        winddirection: number;
        weathercode: number;
        time: string;
    };
    hourly: {
        time: string[];
        temperature_2m: number[];
        precipitation: number[];
        relative_humidity_2m: number[];
        windspeed_10m: number[];
        winddirection_10m: number[];
        uv_index: number[];
        surface_pressure: number[];
        cloudcover: number[];
    };
    daily: {
        time: string[];
        temperature_2m_max: number[];
        temperature_2m_min: number[];
        precipitation_sum: number[];
    };
};

function WeatherDashboard({ nextWindow, locations, setLocations }: WeatherDashboardProps) {
    const [locationList, setLocationList] = useState(locations);
    const [weatherData, setWeatherData] = useState<WeatherData | null>(null);
    const [imgData, setImgData] = useState<string>('');

    const location = locationList[0].name;

    useEffect(() => {
        const fetchWeather = async () => {
            const lat = locationList[0].lat;
            const lon = locationList[0].lon;
            const url = `https://api.open-meteo.com/v1/forecast?latitude=${lat}&longitude=${lon}&current_weather=true&hourly=temperature_2m,precipitation,relative_humidity_2m,windspeed_10m,winddirection_10m,uv_index,surface_pressure,cloudcover&daily=temperature_2m_max,temperature_2m_min,precipitation_sum&timezone=Europe%2FPrague`;

            const res = await fetch(url);
            const data = await res.json();
            setWeatherData(data);
        };

        fetchWeather();
    }, [locationList]);

    useEffect(() => {
        const fetchImgData = async () => {
            try {
                const unsplashKey = 'QomxOmr0uAe3rY0cL076U6MDaOCaBWfrd0DhQjmQCIo';
                const response = await fetch(
                    `https://api.unsplash.com/search/photos?query=${location}&client_id=${unsplashKey}`,
                );

                if (!response.ok) {
                    throw new Error(`HTTP error: ${response.status}`);
                }

                const imgData = await response.json();
                const imgUrl = imgData.results?.[0]?.urls?.regular;

                setImgData(imgUrl || 'https://cdn-icons-png.flaticon.com/512/69/69524.png');
            } catch {
                setImgData('https://cdn-icons-png.flaticon.com/512/69/69524.png');
            }
        };

        fetchImgData();
    }, [location]);

    if (!weatherData || imgData === '') {
        return <div className="p-8">Loading...</div>;
    }

    const current = weatherData.current_weather;
    const hourly = weatherData.hourly;
    const daily = weatherData.daily;

    return (
        <div className="h-full overflow-y-scroll">
            <div className="flex flex-rows gap-8 justify-center items-center">
                <div className="flex-row-1 p-4">
                    <div className="text-xl font-semibold mb-4">{location}</div>

                    {/* Aktuální počasí */}
                    <div className="flex items-center justify-between bg-gray-200 p-4 rounded-xl mb-4 hover:scale-105 transform transition-all duration-300 ease-in-out">
                        <div>
                            <div className="flex items-center space-x-2 text-4xl font-bold">
                                <span>{current.temperature}°C</span>
                                <span>
                                    {(() => {
                                        const now = new Date();
                                        const currentHour = now.getHours();
                                        const rain = hourly.precipitation[currentHour];
                                        const cloudiness = hourly.cloudcover[currentHour];

                                        let icon = '☀️';
                                        if (rain > 5) icon = '🌧️';
                                        else if (rain > 0) icon = '🌦️';
                                        else if (cloudiness > 60) icon = '☁️';
                                        else if (cloudiness > 20) icon = '⛅';

                                        return icon;
                                    })()}
                                </span>
                            </div>
                            <div className="text-sm">
                                Highest: {daily.temperature_2m_max[0]}°C • Lowest: {daily.temperature_2m_min[0]}°C
                            </div>
                        </div>
                    </div>

                    {/* Hodinová předpověď */}
                    <div className="bg-gray-200 p-4 rounded-xl mb-4 min-h-[150px] hover:scale-105 transform transition-all duration-300 ease-in-out">
                        <h2 className="text-lg font-semibold mb-1">Hourly forecast</h2>
                        <div className="flex space-x-1">
                            {(() => {
                                const plusHours = 8;
                                const now = new Date();
                                const currentHour = now.getHours();

                                const temps = hourly.temperature_2m.slice(currentHour, currentHour + plusHours);
                                const hours = hourly.time.slice(currentHour, currentHour + plusHours);
                                const precipitation = hourly.precipitation.slice(currentHour, currentHour + plusHours);
                                const cloud = hourly.cloudcover.slice(currentHour, currentHour + plusHours);

                                return temps.map((temp: number, i: number) => {
                                    const hour = new Date(hours[i]).getHours();
                                    const rain = precipitation[i];
                                    const cloudiness = cloud[i];

                                    let icon = '☀️';
                                    if (rain > 5) icon = '🌧️';
                                    else if (rain > 0) icon = '🌦️';
                                    else if (cloudiness > 60) icon = '☁️';
                                    else if (cloudiness > 20) icon = '⛅';

                                    return (
                                        <div
                                            key={i}
                                            className="flex flex-col items-center min-w-[70px] text-center"
                                        >
                                            <div className="text-xs mb-2 mr-1 text-gray-700">{hour}:00</div>
                                            <div className="text-lg mr-1">{icon}</div>
                                            <div className="text-sm font-medium mt-2">{temp}°C</div>
                                            <div
                                                className="text-sm text-blue-400 mt-1"
                                                style={{ padding: '2px 5px', borderRadius: '5px' }}
                                            >
                                                {rain.toFixed(1)} mm
                                            </div>
                                        </div>
                                    );
                                });
                            })()}
                        </div>
                    </div>

                    {/* Detailní podmínky */}
                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
                        <ConditionCard
                            title="Wind"
                            value={`${current.windspeed} km/h`}
                            sub="10 m above ground"
                        />
                        <ConditionCard
                            title="Wind direction"
                            value={
                                <div className="flex flex-col items-center justify-center">
                                    <div
                                        className="text-2xl"
                                        style={{ transform: `rotate(${hourly.winddirection_10m[0]}deg)` }}
                                    >
                                        ↑
                                    </div>
                                </div>
                            }
                            sub={`${hourly.winddirection_10m[0]}°`}
                        />
                        <ConditionCard
                            title="Pressure"
                            value={`${hourly.surface_pressure[0]} hPa`}
                            sub="Current"
                        />
                        <ConditionCard
                            title="UV Index"
                            value={`${hourly.uv_index[0]}`}
                            sub="Current"
                        />
                        <ConditionCard
                            title="Humidity"
                            value={`${hourly.relative_humidity_2m[0]}%`}
                            sub="Relative"
                        />
                        <ConditionCard
                            title="Cloudiness"
                            value={`${hourly.cloudcover[0]}%`}
                            sub="Current"
                        />
                        <ConditionCard
                            title="Precipitation"
                            value={`${hourly.precipitation[0]} mm`}
                            sub="Hourly estimate"
                        />
                    </div>

                    {/* Denní předpověď */}
                    <div className="bg-gray-200 p-4 rounded-xl hover:scale-105 transform transition-all duration-300 ease-in-out">
                        <h2 className="text-lg font-semibold mb-4">Daily forecast</h2>
                        <table className="w-full text-sm table-auto border-collapse">
                            <thead>
                                <tr className="text-left text-gray-700 border-b border-gray-700">
                                    <th className="py-1 pr-4">Day</th>
                                    <th className="py-1 pr-4">Weather</th>
                                    <th className="py-1 pr-4">Temperature</th>
                                    <th className="py-1">Precipitation</th>
                                </tr>
                            </thead>
                            <tbody>
                                {daily.time.map((day: string, i: number) => {
                                    const Day = new Date(day).toLocaleDateString('en-US', { weekday: 'long' });

                                    const avgCloud = Math.round(
                                        hourly.cloudcover
                                            .slice(i * 24, i * 24 + 24)
                                            .reduce((a: number, b: number) => a + b, 0) / 24,
                                    );

                                    const max = daily.temperature_2m_max[i];
                                    const min = daily.temperature_2m_min[i];
                                    const precipitation = daily.precipitation_sum[i];

                                    let icon = '☀️';
                                    let description = 'Clear';
                                    if (precipitation > 5) {
                                        icon = '🌧️';
                                        description = 'Heavy rain';
                                    } else if (precipitation > 0) {
                                        icon = '🌦️';
                                        description = 'Showers';
                                    } else if (avgCloud > 80) {
                                        icon = '☁️';
                                        description = 'Overcast';
                                    } else if (avgCloud > 40) {
                                        icon = '⛅';
                                        description = 'Partly cloudy';
                                    }

                                    return (
                                        <tr
                                            key={i}
                                            className="border-b border-gray-700"
                                        >
                                            <td className="py-2 pr-4 font-medium">{Day}</td>
                                            <td className="py-2 pr-4">
                                                <span className="text-lg mr-1">{icon}</span>
                                                {description}
                                            </td>
                                            <td className="py-2 pr-4">
                                                {max}°C / {min}°C
                                            </td>
                                            <td className="py-2">{precipitation > 0 ? `${precipitation} mm` : '—'}</td>
                                        </tr>
                                    );
                                })}
                            </tbody>
                        </table>
                    </div>
                </div>

                {/* Pravý panel - obrázek a seznam lokací */}
                <div className="w-[250px] flex flex-col items-center p-4 bg-gray-200 rounded-lg">
                    <div>
                        <img
                            src={imgData}
                            alt={`City ${location}`}
                            className="w-[300vw] h-[300vw] max-w-[200px] max-h-[200px] flex-shrink-0 rounded-xl object-cover margin-4 shadow-md mb-4"
                        />
                    </div>

                    {locationList.map((loc, i) => (
                        <div
                            key={i}
                            className={`relative w-full py-2 px-4 mb-3 rounded-full cursor-pointer transition shadow-xl border-2 border-black flex justify-between items-center hover:scale-105 
            ${i === 0 ? 'bg-gray-500 text-white' : 'hover:bg-gray-500 hover:text-white'}
        `}
                        >
                            {/* Kliknutí na název lokace přehodí lokaci na první místo */}
                            <span
                                className="flex-1 text-center"
                                onClick={() => {
                                    const newList = [loc, ...locationList.filter((l) => l.name !== loc.name)];
                                    setLocationList(newList);
                                    setLocations(newList);
                                    setWeatherData(null);
                                    setImgData('');
                                }}
                            >
                                {loc.name}
                            </span>

                            {/* Popelnice na odstranění */}
                            <span
                                className="absolute right-4 ml-2 text-red-500 hover:text-red-700 cursor-pointer text-lg"
                                onClick={(e) => {
                                    e.stopPropagation();
                                    const index = locationList.findIndex((l) => l.name === loc.name);
                                    if (index !== -1) {
                                        const newList = [...locationList];
                                        newList.splice(index, 1);
                                        setLocationList(newList);
                                        setLocations(newList);

                                        if (newList.length === 0) {
                                            nextWindow();
                                        }
                                    }
                                }}
                                title="Delete location"
                            >
                                🗑️
                            </span>
                        </div>
                    ))}

                    <div
                        className="w-full font-bold text-center py-2 px-4 mb-3 rounded-full cursor-pointer transition shadow-xl border-gray-500 border-2 border-black hover:scale-105 hover:bg-gray-500"
                        onClick={nextWindow}
                        title="Add location"
                    >
                        +
                    </div>
                </div>
            </div>
        </div>
    );
}

type ConditionCardProps = {
    title: string;
    value: string | React.ReactNode;
    sub: string;
};

function ConditionCard({ title, value, sub }: ConditionCardProps) {
    return (
        <div className="bg-gray-200 p-4 rounded-xl text-center hover:scale-105 transform transition-all duration-300 ease-in-out">
            <div className="text-sm text-gray-800">{title}</div>
            <div className="text-xl font-bold">{value}</div>
            <div className="text-xs text-gray-700">{sub}</div>
        </div>
    );
}

type Location = {
    name: string;
    lat: number;
    lon: number;
};

type WeatherDashboardProps = {
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

    // className="bg-gray-900   w-screen   font-sans"
    return (
        <div>
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
