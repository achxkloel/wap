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
