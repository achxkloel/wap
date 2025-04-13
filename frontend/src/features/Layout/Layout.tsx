import Sidebar from '@/features/Sidebar';
import { Outlet } from 'react-router';

function Layout() {
    return (
        <div className="flex h-screen">
            <Sidebar />
            <div className="flex-1 flex flex-col">
                <Outlet />
            </div>
        </div>
    );
}

export default Layout;
