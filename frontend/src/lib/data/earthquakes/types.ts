import { BBox, FeatureCollection, Point } from 'geojson';

export interface EarthquakeProperties {
    mag: number;
    place: string;
    time: number;
    updated: number;
    tz: number;
    url: string;
    detail: string;
    felt: number;
    cdi: number;
    mmi: number;
    alert: string;
    status: string;
    tsunami: number;
    sig: number;
    net: string;
    code: string;
    ids: string;
    sources: string;
    types: string;
    nst: number;
    dmin: number;
    rms: number;
    gap: number;
    magType: string;
    type: string;
}

export type EarthquakeRealtimePeriod = 'hour' | 'day' | 'week' | 'month';
export type EarthquakeRealtimeMagnitude = 'all' | '1.0' | '2.5' | '4.5' | 'significant';

export interface EarthquakeRequestParams {
    // Time
    starttime?: string;
    endtime?: string;
    updatedafter?: string;

    // Location (rectangle)
    minlatitude?: number;
    maxlatitude?: number;
    minlongitude?: number;
    maxlongitude?: number;

    // Location (circle)
    latitude?: number;
    longitude?: number;
    maxradius?: number;
    maxradiuskm?: number;

    // Other
    catalog?: string;
    contributor?: string;
    eventid?: string;
    includeallmagnitudes?: boolean;
    includeallorigins?: boolean;
    includearrivals?: boolean;
    includedeleted?: boolean | 'only';
    includesuperseded?: boolean;
    limit?: number;
    maxdepth?: number;
    maxmagnitude?: number;
    mindepth?: number;
    minmagnitude?: number;
    offset?: number;
    orderby?: string;

    // Extensions
    alertlevel?: string;
    callback?: string;
    eventtype?: string;
    jsonerror?: boolean;
    kmlanimated?: boolean;
    kmlcolorby?: string;
    maxcdi?: number;
    maxgap?: number;
    maxmmi?: number;
    maxsig?: number;
    mincdi?: number;
    minfelt?: number;
    mingap?: number;
    minsig?: number;
    nodata?: number;
    productcode?: string;
    producttype?: string;
    reviewstatus?: string;
}

export interface EarthquakeMetadata {
    generated: number;
    url: string;
    title: string;
    api: string;
    count: number;
    status: number;
}

export interface EarthquakeData extends FeatureCollection<Point, EarthquakeProperties> {
    metadata: EarthquakeMetadata;
    bbox: BBox;
}

export enum EarthquakeEnum {
    CATALOGS = 'catalogs',
    CONTRIBUTORS = 'contributors',
    PRODUCTTYPES = 'producttypes',
    EVENTTYPES = 'eventtypes',
    MAGNITUDETYPES = 'magnitudeTypes',
}

export interface EarthquakeEnumData {
    [EarthquakeEnum.CATALOGS]: string[];
    [EarthquakeEnum.CONTRIBUTORS]: string[];
    [EarthquakeEnum.PRODUCTTYPES]: string[];
    [EarthquakeEnum.EVENTTYPES]: string[];
    [EarthquakeEnum.MAGNITUDETYPES]: string[];
}
