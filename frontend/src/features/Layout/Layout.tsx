import { SidebarProvider } from '@/components/ui/sidebar';
import Sidebar from '@/features/NewSidebar';
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

    // return (
    //     <div className="flex h-screen w-screen overflow-hidden">
    //         <Sidebar />
    //         <div className="flex-1 flex flex-col">
    //             <Outlet />
    //         </div>
    //     </div>
    // );
}

export default Layout;
