import { SidebarProvider } from '@/components/ui/sidebar';
import Sidebar from '@/features/Sidebar';
import { Outlet } from 'react-router';

function Layout() {
    return (
        <SidebarProvider>
            <div className="flex h-screen w-screen overflow-hidden">
                <Sidebar />
                <main className="flex-1 flex flex-col">
                    <Outlet />
                </main>
            </div>
        </SidebarProvider>
    );
}

export default Layout;
