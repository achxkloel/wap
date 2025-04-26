import { Button } from '@/components/ui/button';
import getEarthquakes from '@/lib/data/earthquakes/get';
import getWeather, { current_def, hourly_def } from '@/lib/data/getWeather';
import { getWeatherIconAndDescription } from '@/pages/Weather/WeatherDashboard';
import { ClipboardIcon, PencilIcon, Trash2Icon } from 'lucide-react';
import { useEffect, useState } from 'react';

interface LocationCardProps {
    location: any;
    onEdit?: (location: any) => void;
    onDelete?: (location: any) => void;
}

function LocationCard({ location, onEdit, onDelete }: LocationCardProps) {
    const [hover, setHover] = useState(false);
    const [copySuccess, setCopySuccess] = useState(false);
    const [earthquake, setEarthquake] = useState<{ place: string; magnitude: number } | null>(null);
    const [weather, setWeather] = useState<{ icon: string; description: string } | null>(null);
    let copyTimeout: ReturnType<typeof setTimeout> | null = null;

    useEffect(() => {
        return () => {
            if (copyTimeout) clearTimeout(copyTimeout);
        };
    }, []);

    useEffect(() => {
        if (location) {
            getLocationWeather(location);
            getLocationEarthquakes(location);
        }
    }, [location]);

    const getLocationWeather = async (location: any) => {
        return getWeather({
            latitude: location.latitude,
            longitude: location.longitude,
            current_weather: true,
            hourly: 'precipitation,cloudcover',
            timezone: 'Europe/Prague',
        }).then((data) => {
            const now = new Date();
            const currentHour = now.getHours();
            const hourly = data.hourly ?? hourly_def;
            const current = data.current_weather ?? current_def;

            const desc = getWeatherIconAndDescription(
                hourly.precipitation[currentHour],
                hourly.cloudcover[currentHour],
                current.temperature,
                new Date().getHours(),
            );

            setWeather(desc);
        });
    };

    const getLocationEarthquakes = async (location: any) => {
        return getEarthquakes({
            latitude: location.latitude,
            longitude: location.longitude,
            maxradiuskm: location.radius,
            starttime: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(),
            endtime: new Date().toISOString(),
        }).then((data) => {
            if (!data || !data.features || data.features.length <= 0) {
                return;
            }

            const mostSignificant = data.features.reduce((acc, feature) => {
                if (feature.properties.mag > acc.properties.mag) {
                    return feature;
                }

                return acc;
            }, data.features[0]);

            setEarthquake({
                place: mostSignificant.properties.place,
                magnitude: mostSignificant.properties.mag,
            });
        });
    };

    return (
        <div
            className="flex gap-4 border rounded-lg shadow-md p-4 hover:shadow-lg transition-shadow duration-200 ease-in-out bg-sidebar"
            onMouseEnter={() => setHover(true)}
            onMouseLeave={() => setHover(false)}
        >
            <div className="flex-shrink-0">
                <img
                    src={location.image || undefined}
                    alt={location.name}
                    className="w-[200px] h-[200px] rounded-md object-cover"
                />
            </div>
            <div className="flex-1 flex flex-col gap-4 justify-between">
                <div className="flex flex-col">
                    <div className="flex justify-between items-start">
                        <h2 className="text-2xl font-bold pb-1">{location.name}</h2>
                        {hover && (
                            <div className="flex gap-2">
                                <Button
                                    variant="outline"
                                    size="icon"
                                    className="p-2 dark:hover:bg-black"
                                    onClick={() => {
                                        if (onEdit) {
                                            onEdit(location);
                                        }
                                    }}
                                >
                                    <PencilIcon className="size-4" />
                                </Button>
                                <Button
                                    variant="destructive"
                                    size="icon"
                                    className="p-2"
                                    onClick={() => {
                                        if (onDelete) {
                                            onDelete(location);
                                        }
                                    }}
                                >
                                    <Trash2Icon className="size-4" />
                                </Button>
                            </div>
                        )}
                    </div>
                    <p className="text-gray-500 dark:text-gray-50">{location.description}</p>
                </div>
                <div className="grid grid-cols-2">
                    <div>
                        <div>Coordinates</div>
                        <div>Radius</div>
                        <div>Weather</div>
                        <div>Earthquake</div>
                        {earthquake && <div>Magnitude</div>}
                    </div>

                    <div>
                        <div className="flex items-center gap-2">
                            {location.latitude.toFixed(2)}, {location.longitude.toFixed(2)}{' '}
                            <ClipboardIcon
                                className="w-6 h-6 hover:bg-muted cursor-pointer p-1 rounded"
                                onClick={() => {
                                    if (!('clipboard' in navigator)) {
                                        return;
                                    }

                                    if (copyTimeout) clearTimeout(copyTimeout);
                                    setCopySuccess(true);
                                    copyTimeout = setTimeout(() => setCopySuccess(false), 1000);

                                    navigator.clipboard.writeText(`${location.latitude}, ${location.longitude}`);
                                }}
                            />
                            {copySuccess && <span className="text-gray-500 dark:text-gray-50 text-sm">Copied!</span>}
                        </div>
                        <div>{location.radius} km</div>
                        <div>{weather ? `${weather.description} ${weather.icon}` : ''}</div>
                        <div>{earthquake ? `${earthquake.place}` : 'No'}</div>
                        {earthquake && <div>{Math.floor(earthquake.magnitude)}</div>}
                    </div>
                </div>
            </div>
        </div>
    );
}

export default LocationCard;
