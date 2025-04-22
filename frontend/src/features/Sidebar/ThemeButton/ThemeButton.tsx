import { SidebarMenu, SidebarMenuItem } from '@/components/ui/sidebar';
import { Switch } from '@/components/ui/switch';
import usePreferences from '@/lib/store/preferences';
import { useEffect } from 'react';

function ThemeButton() {
    const theme = usePreferences((state) => state.theme);
    const setTheme = usePreferences((state) => state.setTheme);

    useEffect(() => {
        const root = window.document.documentElement;

        root.classList.remove('light', 'dark');

        if (theme === 'system') {
            const systemTheme = getSystemTheme();

            root.classList.add(systemTheme);
            return;
        }

        root.classList.add(theme);
    }, [theme]);

    const getSystemTheme = () => {
        return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    };

    const handleModeChange = () => {
        setTheme(theme === 'dark' ? 'light' : 'dark');
    };

    const isDarkMode = () => {
        if (theme === 'system') {
            return getSystemTheme() === 'dark';
        }

        return theme === 'dark';
    };

    return (
        <SidebarMenu>
            <SidebarMenuItem
                onClick={handleModeChange}
                className="flex gap-3 items-center w-full p-2 rounded hover:bg-muted cursor-pointer"
            >
                <div className="flex-1">Dark mode</div>
                <Switch
                    onClick={(e) => e.preventDefault()}
                    checked={isDarkMode()}
                />
            </SidebarMenuItem>
        </SidebarMenu>
    );
}

export default ThemeButton;
