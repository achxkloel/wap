const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

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
    debug: [
        // PRINT.VERBOSE,
        PRINT.DEBUG,
        // PRINT.STORE,
        PRINT.INFO,
        PRINT.WARN,
        PRINT.ERROR,
    ],
};
