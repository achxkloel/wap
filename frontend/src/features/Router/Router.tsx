import Layout from '@/features/Layout';
import { checkToken } from '@/lib/api';
import Locations from '@/pages/Locations';
import Map from '@/pages/Map';
import Settings from '@/pages/Settings';
import React, { useEffect } from 'react';
import { Route, Routes, useLocation } from 'react-router';
import ProtectedRoute from './ProtectedRoute';
import Weather from '@/pages/Weather';

function Router() {
    const location = useLocation();

    useEffect(() => {
        checkToken();
    }, [location]);

    return (
        <Routes>
            <Route element={<Layout />}>
                <Route
                    path="/"
                    element={<Weather />}
                />
                <Route
                    path="/map"
                    element={<Map />}
                />

                <Route element={<ProtectedRoute />}>
                    <Route
                        path="/locations"
                        element={<Locations />}
                    />
                    <Route
                        path="/settings"
                        element={<Settings />}
                    />
                </Route>
            </Route>
        </Routes>
    );
}

export default Router;
