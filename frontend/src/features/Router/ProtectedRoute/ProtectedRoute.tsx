import useAuthStore from '@/lib/store/auth';
import { Navigate, Outlet } from 'react-router';

function ProtectedRoute() {
    const token = useAuthStore((state) => state.token);
    return token !== null ? <Outlet /> : <Navigate to="/" />;
}

export default ProtectedRoute;
