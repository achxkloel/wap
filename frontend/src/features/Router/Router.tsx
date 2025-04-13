import Layout from '@/features/Layout';
import Map from '@/pages/Map';
import Settings from '@/pages/Settings';
import React from 'react';
import { Route, Routes } from 'react-router';
import ProtectedRoute from './ProtectedRoute';

function Router() {
    return (
        <Routes>
            <Route element={<Layout />}>
                <Route
                    path="/"
                    element={<React.Fragment />}
                />
                <Route
                    path="/locations"
                    element={<React.Fragment />}
                />
                <Route
                    path="/map"
                    element={<Map />}
                />

                <Route element={<ProtectedRoute />}>
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
