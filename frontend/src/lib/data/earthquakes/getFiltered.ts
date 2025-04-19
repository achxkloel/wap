import { FilterFormValues } from '@/pages/Map/Filters/Filters';
import get from './get';
import getRealtime from './getRealtime';
import { EarthquakeRequestParams } from './types';

export const getFiltered = (filters: FilterFormValues) => {
    if (filters.mode === 'realtime') {
        return getRealtime(filters.realtimePeriod, filters.realtimeMagnitude);
    }

    const params: EarthquakeRequestParams = {
        starttime: filters.startTime ? filters.startTime.toISOString() : undefined,
        endtime: filters.endTime ? filters.endTime.toISOString() : undefined,
        minlatitude: filters.minLatitude,
        maxlatitude: filters.maxLatitude,
        minlongitude: filters.minLongitude,
        maxlongitude: filters.maxLongitude,
        latitude: filters.latitude,
        longitude: filters.longitude,
        maxradiuskm: filters.maxRadiusKm,
        catalog: filters.catalog,
        contributor: filters.contributor,
        includeallmagnitudes: filters.includeAllMagnitudes,
        includeallorigins: filters.includeAllOrigins,
        includearrivals: filters.includeAllArrivals,
        maxdepth: filters.maxDepth,
        maxmagnitude: filters.maxMagnitude,
        mindepth: filters.minDepth,
        minmagnitude: filters.minMagnitude,
        producttype: filters.productType,
    };

    return get(params);
};

export default getFiltered;
