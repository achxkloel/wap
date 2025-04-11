import {environment, PRINT} from "@/environment/environment.ts";

/**
 * Singleton class for logging
 */
class Logger {
    private static _instance: Logger;

    private constructor() {}

    static getInstance() {
        if (!Logger._instance) {
            Logger._instance = new Logger();
        }
        return Logger._instance;
    }

    verbose(...args: any[]) {
        if (environment.debug.includes(PRINT.VERBOSE)) {
            console.info(...args);
        }
    }

    debug(...args: any[]) {
        if (environment.debug.includes(PRINT.DEBUG)) {
            console.debug(...args);
        }
    }

    store(...args: any[]) {
        if (environment.debug.includes(PRINT.STORE)) {
            console.debug(...args);
        }
    }

    info(...args: any[]) {
        if (environment.debug.includes(PRINT.INFO)) {
            console.info(...args);
        }
    }

    warn(...args: any[]) {
        if (environment.debug.includes(PRINT.WARN)) {
            console.warn(...args);
        }
    }

    error(...args: any[]) {
        if (environment.debug.includes(PRINT.ERROR)) {
            console.error(...args);
        }
    }

    assert(condition: boolean, ...message: any[]) {
        console.assert(condition, message);
    }
}

export const logger = Logger.getInstance();

