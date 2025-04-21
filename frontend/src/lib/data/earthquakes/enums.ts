import { api } from './api';
import { EarthquakeEnum, EarthquakeEnumData } from './types';

const getEnum = async (item: EarthquakeEnum) => {
    return api.get<EarthquakeEnumData>(`/application.json`).then((response) => response.data[item]);
};

export const getCatalogEnum = () => getEnum(EarthquakeEnum.CATALOGS);
export const getContributorEnum = () => getEnum(EarthquakeEnum.CONTRIBUTORS);
export const getProductTypeEnum = () => getEnum(EarthquakeEnum.PRODUCTTYPES);
export const getEventTypeEnum = () => getEnum(EarthquakeEnum.EVENTTYPES);
export const getMagnitudeTypeEnum = () => getEnum(EarthquakeEnum.MAGNITUDETYPES);
