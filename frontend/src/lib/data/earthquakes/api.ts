import axios from 'axios';

export const api = axios.create({
    baseURL: 'https://earthquake.usgs.gov/fdsnws/event/1',
});

export const realtimeApi = axios.create({
    baseURL: 'https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary',
});
