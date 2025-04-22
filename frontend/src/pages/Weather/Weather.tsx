import React, { useState, useEffect } from 'react';

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
};

const CityCard: React.FC<CityCardProps> = ({ name }) => {
    const [image, setImage] = useState<string>('');
    const [temp, setTemp] = useState<string>('?');
    const [rain, setRain] = useState<string>('?');
    const [wind, setWind] = useState<string>('?');

    useEffect(() => {
        const fetchData = async () => {
            // Zjisti souřadnice z názvu města
            const geoRes = await fetch(`https://geocoding-api.open-meteo.com/v1/search?name=${name}&count=1`);
            const geoData = await geoRes.json();
            const lat = geoData.results?.[0]?.latitude;
            const lon = geoData.results?.[0]?.longitude;

            if (lat && lon) {
                // Získej Current Weather
                const weatherRes = await fetch(
                    `https://api.open-meteo.com/v1/forecast?latitude=${lat}&longitude=${lon}&current_weather=true&daily=precipitation_sum&timezone=Europe%2FPrague`,
                );
                const weatherData = await weatherRes.json();
                const current = weatherData.current_weather;
                setTemp(current.temperature.toFixed(1));
                setWind(current.windspeed.toFixed(0));

                const dailyRain = weatherData.daily?.precipitation_sum?.[0];
                setRain(dailyRain ? dailyRain.toFixed(1) : '0');
            }

            // Najdi obrázek města
            if (name) {
                const unsplashKey = 'QomxOmr0uAe3rY0cL076U6MDaOCaBWfrd0DhQjmQCIo';
                const imgRes = await fetch(
                    `https://api.unsplash.com/search/photos?query=${encodeURIComponent(name)}&client_id=${unsplashKey}`,
                );

                const imgData = await imgRes.json();
                const imgUrl = imgData.results?.[0]?.urls?.regular;

                if (imgUrl) setImage(imgUrl);
                else setImage('https://placehold.co/300x200/eeeeee/444444?text=No+Image');
            } else {
                setImage('https://placehold.co/300x200/eeeeee/444444?text=No+Image');
            }
        };

        fetchData();
    }, [name]);

    return (
        <div className="p-4 flex flex-col items-center bg-gray-200 rounded-lg shadow-xl hover:scale-105 transform transition-all duration-300 ease-in-out">
            <img
                src={image}
                alt={`City ${name}`}
                className="w-[300vw] h-[300vw] max-w-[200px] max-h-[200px] flex-shrink-0 rounded-xl object-cover shadow-md mb-4"
            />

            <div className="text-sm  mt-2 text-center">
                <p className=" text-lg font-bold">{name}</p>

                <p className="text-gray-800">
                    Temperature: <span className=" font-semibold">{temp}°C</span>
                </p>
                <p className="text-blue-800">
                    Precipitation: <span className=" font-semibold">{rain} mm</span>
                </p>
                <p className="text-green-800">
                    Wind: <span className=" font-semibold">{wind} km/h</span>
                </p>
            </div>
        </div>
    );
};

const WeatherNoSelect = ({ onGetLocation }: { onGetLocation: () => void }) => {
    const randomCities = allCities.sort(() => 0.5 - Math.random()).slice(0, 4);
    // w-screen h-screen bg-gray-900
    return (
        <div className=" flex justify-center items-center min-h-screen">
            <div className="grid grid-cols-3  gap-8">
                {/* 4 rohové karty */}
                <div className="col-start-1 row-start-1 flex justify-center items-center">
                    <CityCard name={randomCities[0]!} />
                </div>

                <div className="col-start-3 row-start-1 flex justify-center items-center">
                    <CityCard name={randomCities[1]!} />
                </div>

                <div className="col-start-1 row-start-3 flex justify-center items-center">
                    <CityCard name={randomCities[2]!} />
                </div>

                <div className="col-start-3 row-start-3 flex justify-center items-center">
                    <CityCard name={randomCities[3]!} />
                </div>

                {/* Střed */}
                <div className="col-start-2 row-start-2 flex justify-center items-center flex-col">
                    <div className="text-xl font-semibold mb-2 text-gray-800">No location selected</div>
                    <br />
                    <div
                        className="  px-6 py-3 rounded-full border-2 border-gray-700 text-xl shadow-xl font-semibold shadow-lg cursor-pointer hover:bg-gray-200 transform transition-all duration-300 ease-in-out hover:scale-105"
                        onClick={onGetLocation}
                    >
                        Location selection
                    </div>
                </div>
            </div>
        </div>
    );
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

function WeatherDashboard({ onBack }: { onBack: () => void }) {
    const [weatherData, setWeatherData] = useState<WeatherData | null>(null);
    const [imgData, setImgData] = useState<string>('');
    const location = 'Prague';

    useEffect(() => {
        const fetchWeather = async () => {
            const lat = 50.0101; // Praha  přibližně
            const lon = 14.45;
            const url = `https://api.open-meteo.com/v1/forecast?latitude=${lat}&longitude=${lon}&current_weather=true&hourly=temperature_2m,precipitation,relative_humidity_2m,windspeed_10m,winddirection_10m,uv_index,surface_pressure,cloudcover&daily=temperature_2m_max,temperature_2m_min,precipitation_sum&timezone=Europe%2FPrague`;

            const res = await fetch(url);
            const data = await res.json();
            setWeatherData(data);
        };

        fetchWeather();
    }, []);

    useEffect(() => {
        const fetchImgData = async () => {
            const unsplashKey = 'QomxOmr0uAe3rY0cL076U6MDaOCaBWfrd0DhQjmQCIo';
            const imgRes = await fetch(
                `https://api.unsplash.com/search/photos?query=${location}&client_id=${unsplashKey}`,
            );
            const imgData = await imgRes.json();
            const imgUrl = imgData.results?.[0]?.urls?.regular;

            if (imgUrl) setImgData(imgUrl);
            else setImgData('https://via.placeholder.com/300x200?text=No+Image');
        };

        fetchImgData();
    }, []);

    if (!weatherData || imgData == '') {
        return <div className=" p-8">Loading...</div>;
    }

    const current = weatherData.current_weather;
    const hourly = weatherData.hourly;
    const daily = weatherData.daily;

    return (
        <div className="flex flex-rows overflow gap-8  justify-center items-center ">
            <div className="  flex-row-1 p-4  overflow-x-auto">
                <div className="text-xl font-semibold mb-4">{location}</div>

                <div className="flex items-center justify-between bg-gray-200 p-4 rounded-xl mb-4 hover:scale-105 transform transition-all duration-300 ease-in-out">
                    <div>
                        <div className="text-4xl font-bold">{current.temperature}°C</div>
                        <div className="text-sm">
                            Highest: {daily.temperature_2m_max[0]}°C • Lowest: {daily.temperature_2m_min[0]}°C
                        </div>
                    </div>
                </div>

                <div className="bg-gray-200 p-4 rounded-xl mb-4 overflow-x-auto  min-h-[150px] hover:scale-105 transform transition-all duration-300 ease-in-out">
                    <h2 className="text-lg font-semibold mb-1">Hourly forecast</h2>
                    <div className="flex space-x-1 ">
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
                                        <div className="text-lg mr-1">{icon}</div>
                                        <div className="text-sm font-medium mt-2">{temp}°C</div>
                                        {
                                            <div
                                                className="text-sm text-blue-400 mt-1"
                                                style={{ padding: '2px 5px', borderRadius: '5px' }}
                                            >
                                                {rain.toFixed(1)} mm
                                            </div>
                                        }
                                        <div className="text-xs mt-1 text-gray-700">{hour}:00</div>
                                    </div>
                                );
                            });
                        })()}
                    </div>
                </div>

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
                                    title={`Směr: ${hourly.winddirection_10m[0]}°C`}
                                >
                                    ↑
                                </div>
                            </div>
                        }
                        sub={`${hourly.winddirection_10m[0]}°C`}
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
                        title="Vlhkost"
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

                <div className="bg-gray-200 p-4 rounded-xl overflow-x-auto hover:scale-105 transform transition-all duration-300 ease-in-out ">
                    <h2 className="text-lg font-semibold mb-4">Dayní předpověď</h2>
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
                                const Day = new Date(day).toLocaleDateString('en-US', {
                                    weekday: 'long',
                                });

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

            {/* Pravý panel - obrázek + lokace */}
            <div className="w-[250px] flex  flex-col items-center p-4  bg-gray-200 rounded-lg ">
                <div>
                    <img
                        src={imgData}
                        alt={`City ${location}`}
                        className="w-[300vw] h-[300vw] max-w-[200px] max-h-[200px] flex-shrink-0 rounded-xl object-cover margin-4 shadow-md mb-4"
                    />
                </div>

                {[location, 'Praha 1', 'Praha 5', 'Praha 6'].map((loc, i) => (
                    <div
                        key={i}
                        className={`w-full text-center py-2 px-4 mb-3 rounded-full  cursor-pointer transition shadow-xl border-gray-500 border-2 border-black hover:scale-105 
            ${i === 0 ? 'bg-gray-500 ' : ' hover:bg-gray-500'}
          `}
                    >
                        {loc}
                    </div>
                ))}

                <div
                    className="w-full text-center py-2 px-4 mb-3 rounded-full  cursor-pointer transition shadow-xl border-gray-500 border-2 border-black hover:scale-105 hover:bg-gray-500"
                    onClick={onBack}
                >
                    +
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

function Weather() {
    const [showDashboard, setShowDashboard] = useState(false);

    // className="bg-gray-900   w-screen   font-sans"
    return (
        <div>
            {showDashboard ? (
                <WeatherDashboard onBack={() => setShowDashboard(false)} />
            ) : (
                <WeatherNoSelect onGetLocation={() => setShowDashboard(true)} />
            )}
        </div>
    );
}

export default Weather;
