import Layout from '@/features/Layout';
import Main from '@/pages/Main';
import Map from '@/pages/Map';
import Settings from '@/pages/Settings';
import { Route, Routes } from 'react-router';

function Router() {
    return (
        <Routes>
            <Route element={<Layout />}>
                <Route
                    path="/"
                    element={<Main />}
                />
                <Route
                    path="/:id"
                    element={<Main />}
                />
                <Route
                    path="/settings"
                    element={<Settings />}
                />
                <Route
                    path="/map"
                    element={<Map />}
                />
            </Route>
        </Routes>
    );
}

export default Router;
