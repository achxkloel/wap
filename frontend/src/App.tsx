import Router from '@/features/Router';
import { useEffect } from 'react';
import api from './lib/api';
import { logger } from './lib/logger';
import { isAuthorized } from './lib/store/auth';

function App() {
    useEffect(() => {
        if (!isAuthorized()) {
            return;
        }

        fetchUser();
    }, []);

    const fetchUser = async () => {
        try {
            logger.debug('Fetching user data...');
            const res = await api.post('/auth/me');
            logger.debug('User data fetched successfully:', res.data);
        } catch (error) {
            logger.error('Error fetching user data:', error);
        }
    };

    return <Router />;
}

export default App;
