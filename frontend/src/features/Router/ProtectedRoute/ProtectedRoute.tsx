import { useIsAuthorized } from '@/lib/store/auth';
import { Navigate, Outlet } from 'react-router';

function ProtectedRoute() {
    const isAuthorized = useIsAuthorized();
    return isAuthorized ? <Outlet /> : <Navigate to="/" />;
}

export default ProtectedRoute;
