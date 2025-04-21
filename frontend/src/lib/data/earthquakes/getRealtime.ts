import { realtimeApi } from './api';
import { EarthquakeData, EarthquakeRealtimeMagnitude, EarthquakeRealtimePeriod } from './types';

export const getRealtime = async (period: EarthquakeRealtimePeriod, magnitude: EarthquakeRealtimeMagnitude) => {
    return realtimeApi<EarthquakeData>(`/${magnitude}_${period}.geojson`).then((response) => response.data);
};

export default getRealtime;
