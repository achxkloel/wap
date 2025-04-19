import { api } from './api';
import { EarthquakeData, EarthquakeRequestParams } from './types';

export const get = async (params: EarthquakeRequestParams) => {
    return api
        .get<EarthquakeData>(`/query`, {
            params: {
                format: 'geojson',
                ...params,
            },
        })
        .then((response) => response.data);
};

export default get;
