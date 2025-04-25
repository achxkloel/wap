import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export const getErrorMessage = (err: unknown, fallback: string) => {
    // Axios-like error shape
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const maybe = err as any;
    return maybe?.response?.data?.data || maybe?.message || fallback;
};

export const numberPreprocess = (val: unknown) => {
    if (typeof val === 'string' && val.trim() === '') {
        return null;
    }

    const parsed = Number(val);
    if (isNaN(parsed)) {
        return val;
    }

    return parsed;
};

export const getCurrentLocation = () => {
    return new Promise<GeolocationPosition>((resolve, reject) => {
        if (!('geolocation' in navigator)) {
            return reject(new Error('Geolocation is not supported by this browser.'));
        }

        navigator.geolocation.getCurrentPosition(resolve, reject);
    });
};
