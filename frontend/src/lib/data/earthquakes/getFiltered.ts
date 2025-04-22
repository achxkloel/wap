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
        minlatitude: filters.minLatitude || undefined,
        maxlatitude: filters.maxLatitude || undefined,
        minlongitude: filters.minLongitude || undefined,
        maxlongitude: filters.maxLongitude || undefined,
        latitude: filters.latitude || undefined,
        longitude: filters.longitude || undefined,
        maxradiuskm: filters.maxRadiusKm || undefined,
        catalog: filters.catalog,
        contributor: filters.contributor,
        includeallmagnitudes: filters.includeAllMagnitudes,
        includeallorigins: filters.includeAllOrigins,
        includearrivals: filters.includeAllArrivals,
        maxdepth: filters.maxDepth || undefined,
        maxmagnitude: filters.maxMagnitude || undefined,
        mindepth: filters.minDepth || undefined,
        minmagnitude: filters.minMagnitude || undefined,
        producttype: filters.productType,
        limit: filters.limit || undefined,
    };

    return get(params);
};

export default getFiltered;
