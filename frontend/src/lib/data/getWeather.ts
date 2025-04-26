import axios from 'axios';

export const current_def = {
    temperature: 0,
    windspeed: 0,
    winddirection: 0,
    weathercode: 0,
    time: '',
};

export const hourly_def = {
    time: [],
    temperature_2m: [],
    precipitation: [],
    relative_humidity_2m: [],
    windspeed_10m: [],
    winddirection_10m: [],
    uv_index: [],
    surface_pressure: [],
    cloudcover: [],
};

export const daily_def = {
    time: [],
    temperature_2m_max: [],
    temperature_2m_min: [],
    precipitation_sum: [],
};

export interface WeatherCurrent {
    temperature: number;
    windspeed: number;
    winddirection: number;
    weathercode: number;
    time: string;
}

export interface WeatherHourly {
    time: string[];
    temperature_2m: number[];
    precipitation: number[];
    relative_humidity_2m: number[];
    windspeed_10m: number[];
    winddirection_10m: number[];
    uv_index: number[];
    surface_pressure: number[];
    cloudcover: number[];
}

export interface WeatherDaily {
    time: string[];
    temperature_2m_max: number[];
    temperature_2m_min: number[];
    precipitation_sum: number[];
}

export interface WeatherData {
    latitude: number;
    longitude: number;
    generationtime_ms: number;
    utc_offset_seconds: number;
    timezone: string;
    timezone_abbreviation: string;
    elevation: number;
    current_weather?: WeatherCurrent;
    hourly?: WeatherHourly;
    daily?: WeatherDaily;
}

export interface WeatherRequestParams {
    latitude: number;
    longitude: number;
    current_weather?: boolean;
    hourly?: string; // comma-separated string, e.g., "temperature_2m,precipitation"
    daily?: string;
    timezone?: string;
}

const WEATHER_URL = 'https://api.open-meteo.com/v1/forecast';

const getWeather = async (params: WeatherRequestParams) => {
    return axios.get(WEATHER_URL, { params }).then((response) => response.data as WeatherData);
};

export default getWeather;
