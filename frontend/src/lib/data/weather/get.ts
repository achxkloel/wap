import { api } from './api';

export const get = async (lat: number, lon: number) => {
    return api
        .get('/forecast?', {
            params: {
                latitude: lat,
                longitude: lon,
                current_weather: true,
                hourly: 'temperature_2m,precipitation,relative_humidity_2m,windspeed_10m,winddirection_10m,uv_index,surface_pressure,cloudcover',
                daily: 'temperature_2m_max,temperature_2m_min,precipitation_sum',
                timezone: 'Europe/Prague',
            },
        })
        .then((res) => res.data);
};

export default get;
