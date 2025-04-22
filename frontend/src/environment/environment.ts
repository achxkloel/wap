const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';
const GOOGLE_CLIENT_ID = import.meta.env.VITE_GOOGLE_CLIENT_ID || '';

export enum PRINT {
    VERBOSE = 1,
    DEBUG = 2,
    STORE = 4,
    INFO = 8,
    WARN = 16,
    ERROR = 32,
}

export const environment = {
    baseUrl: API_BASE_URL,
    googleClientId: GOOGLE_CLIENT_ID,
    debug: [
        // PRINT.VERBOSE,
        PRINT.DEBUG,
        // PRINT.STORE,
        PRINT.INFO,
        PRINT.WARN,
        PRINT.ERROR,
    ],
};
