import axios from 'axios';

export const geo_def = {
    latitude: 0,
    longitude: 0,
    name: '',
    country: '',
};

export interface GeocodingResult {
    name: string;
    country: string;
    latitude: number;
    longitude: number;
}

export interface GeocodingData {
    results: GeocodingResult[];
}

export interface GeocodingRequestParams {
    name: string;
    count?: number;
}

// Base URL for geocoding API
const GEO_URL = 'https://geocoding-api.open-meteo.com/v1/search';

// Fetch geocoding data using axios
const getGeocodingData = async (params: GeocodingRequestParams) => {
    try {
        const response = await axios.get(GEO_URL, { params });
        return response.data as GeocodingData;
    } catch (error) {
        console.error('Error fetching geocoding data:', error);
    }
};

export default getGeocodingData;
