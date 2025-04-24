import {
    Sidebar as SidebarUI,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarGroupContent,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from '@/components/ui/sidebar';
import { useIsAuthorized } from '@/lib/store/auth';
import { CloudSunIcon, LucideIcon, MapIcon, MapPinIcon, SettingsIcon } from 'lucide-react';
import { Link, useLocation } from 'react-router';
import AppTitle from './AppTitle';
import NavGuest from './NavGuest';
import NavUser from './NavUser';
import ThemeButton from './ThemeButton';

interface MenuItem {
    label: string;
    url: string;
    icon: LucideIcon;
    protected?: boolean;
}

const items: MenuItem[] = [
    { label: 'Weather', url: '/', icon: CloudSunIcon },
    { label: 'Locations', url: '/locations', icon: MapPinIcon },
    { label: 'Map', url: '/map', icon: MapIcon },
    { label: 'Settings', url: '/settings', icon: SettingsIcon, protected: true },
];

function Sidebar() {
    const isAuthorized = useIsAuthorized();
    const location = useLocation();

    const isSelected = (path: string) => {
        if (path === '/') {
            return location.pathname === path;
        }

        return location.pathname.startsWith(path);
    };

    return (
        <SidebarUI>
            <SidebarHeader>
                <AppTitle />
            </SidebarHeader>
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            {items.map((item) => {
                                if (item.protected && !isAuthorized) {
                                    return null;
                                }

                                return (
                                    <SidebarMenuItem key={item.label}>
                                        <SidebarMenuButton
                                            asChild
                                            isActive={isSelected(item.url)}
                                        >
                                            <Link to={item.url}>
                                                <item.icon />
                                                <span>{item.label}</span>
                                            </Link>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                );
                            })}
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
            <SidebarFooter>
                <ThemeButton />
                {isAuthorized ? <NavUser /> : <NavGuest />}
            </SidebarFooter>
        </SidebarUI>
    );
}

export default Sidebar;
