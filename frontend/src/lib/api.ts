import { environment } from '@/environment/environment';
import axios from 'axios';
import * as jose from 'jose';
import { logger } from './logger';
import useAuthStore from './store/auth';

const EXPIRATION_THRESHOLD = 60 * 5; // in seconds

const isTokenExpired = (token: string): boolean => {
    const decodedToken = jose.decodeJwt(token);
    const currentTime = Math.floor(Date.now() / 1000);
    const expirationTime = decodedToken.exp;

    if (!expirationTime) {
        return false;
    }

    return expirationTime < currentTime;
};

const willTokenExpire = (token: string): boolean => {
    const decodedToken = jose.decodeJwt(token);
    const currentTime = Math.floor(Date.now() / 1000);
    const expirationTime = decodedToken.exp;

    if (!expirationTime) {
        return false;
    }

    const timeLeft = expirationTime - currentTime;
    return timeLeft < EXPIRATION_THRESHOLD;
};

export const checkToken = () => {
    const accessToken = useAuthStore.getState().access_token;
    const refreshToken = useAuthStore.getState().refresh_token;

    if (!accessToken) {
        return;
    }

    if (isTokenExpired(accessToken)) {
        logger.debug('Access token expired, removing tokens...');
        useAuthStore.getState().removeAccessToken();
        useAuthStore.getState().removeRefreshToken();
        useAuthStore.getState().setUser(null);
        return;
    }

    if (willTokenExpire(accessToken)) {
        logger.debug('Access token is about to expire, refreshing...');

        axios({
            method: 'post',
            url: `${environment.baseUrl}/auth/refresh`,
            headers: {
                Authorization: `Bearer ${refreshToken}`,
            },
        })
            .then((res) => {
                logger.debug('Token refreshed successfully');
                const data = res.data;
                useAuthStore.getState().setAccessToken(data.access_token);
            })
            .catch(() => {
                logger.debug('Failed to refresh token');
            });
    }
};

const api = axios.create({
    baseURL: '/api',
    withCredentials: true,
});

api.interceptors.request.use((config) => {
    checkToken();

    const accessToken = useAuthStore.getState().access_token;

    if (accessToken) {
        config.headers.Authorization = `Bearer ${accessToken}`;
    }

    return config;
});

export default api;
