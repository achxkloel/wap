import { Button } from '@/components/ui/button';
import { useIsAuthorized } from '@/lib/store/auth';
import type { WeatherDashboardProps } from '@/pages/Weather/Weather.tsx';
import { faArrowLeft, faPlus, faTrash } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { useEffect, useState } from 'react';

type ConditionCardProps = {
    title: string;
    value: string | React.ReactNode;
    sub: string;
};

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

function ConditionCard({ title, value, sub }: ConditionCardProps) {
    return (
        <div className="bg-sidebar-border p-4 rounded-xl text-center hover:scale-105 transform transition-all duration-300 ease-in-out">
            <div className="text-sm ">{title}</div>
            <div className="text-xl font-bold">{value}</div>
            <div className="text-xs ">{sub}</div>
        </div>
    );
}

function WeatherDashboard({ nextWindow, locations, setLocations }: WeatherDashboardProps) {
    const [locationList, setLocationList] = useState(locations);
    const [weatherData, setWeatherData] = useState<WeatherData | null>(null);
    const [imgData, setImgData] = useState<string>('');
    const isAuthorized = useIsAuthorized();

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
        <div className="h-full">
            <div className="flex flex-rows gap-8 ">
                <div className="flex-row-1 p-4">
                    <Button
                        className="mb-4"
                        onClick={() => {
                            if (!isAuthorized) setLocations([]);
                            nextWindow();
                        }}
                        variant="outline"
                    >
                        <FontAwesomeIcon
                            icon={faArrowLeft}
                            className="text-sidebar-accent-foreground "
                        />
                    </Button>

                    {/* Aktuální počasí */}
                    <div className="flex items-center justify-between bg-sidebar-border p-4 rounded-xl mb-4 hover:scale-105 transform transition-all duration-300 ease-in-out">
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
                    <div className="bg-sidebar-border p-4 rounded-xl mb-4 min-h-[150px] hover:scale-105 transform transition-all duration-300 ease-in-out">
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
                                            <div className="text-xs mb-2 mr-1 ">{hour}:00</div>
                                            <div className="text-lg mr-1">{icon}</div>
                                            <div className="text-sm font-medium mt-2 text-sidebar-foreground">
                                                {temp}°C
                                            </div>
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
                    <div className="bg-sidebar-border p-4 rounded-xl hover:scale-105 transform transition-all duration-300 ease-in-out">
                        <h2 className="text-lg font-semibold mb-4">Daily forecast</h2>
                        <table className="w-full text-sm table-auto border-collapse">
                            <thead>
                                <tr className="text-left  border-b border-gray-500">
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
                <div className="w-[250px] flex flex-col items-center p-4  ">
                    <div className="text-3xl font-bold mb-4 mt-14">{location}</div>
                    <div className=" flex flex-col items-center p-4 bg-sidebar-border rounded-lg">
                        <div>
                            <img
                                src={imgData}
                                alt={`City ${location}`}
                                className="w-[300vw] h-[300vw] max-w-[200px] max-h-[200px] flex-shrink-0 rounded-xl object-cover margin-4 shadow-md mb-4"
                            />
                        </div>

                        {isAuthorized && (
                            <>
                                {locationList.map((loc, i) => (
                                    <Button
                                        className={'group relative w-full mb-2 shadow-lg'}
                                        {...(i === 0 ? { variant: 'ghost' } : {})}
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            const index = locationList.findIndex((l) => l.name === loc.name);
                                            if (index !== -1 && index !== 0) {
                                                const newList = [...locationList];
                                                newList.splice(index, 1);
                                                newList.unshift(loc);
                                                setLocationList(newList);
                                                setLocations(newList);
                                                setWeatherData(null);
                                                setImgData('');
                                            }
                                        }}
                                    >
                                        <span className="flex-1 text-center">{loc.name}</span>
                                        <span
                                            className="absolute right-4 ml-2 text-red-500 hover:text-red-700 cursor-pointer text-lg hidden group-hover:block"
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
                                            <FontAwesomeIcon icon={faTrash} />
                                        </span>
                                    </Button>
                                ))}

                                <Button
                                    className={'w-full mb-2 shadow-lg font-bold'}
                                    onClick={nextWindow}
                                    title="Add location"
                                >
                                    <FontAwesomeIcon icon={faPlus} />
                                </Button>
                            </>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
}

export default WeatherDashboard;
