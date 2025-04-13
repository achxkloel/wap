import {
    NavigationMenu,
    NavigationMenuItem,
    NavigationMenuList,
    NavigationMenuLink,
} from '@/components/ui/navigation-menu';
import { Button } from '@/components/ui/button';
import useStore from '@/lib/store';
import { Link } from 'react-router-dom';
import { AuthDialog } from '@/components/AuthDialog';

function Header() {
    const counter = useStore((state) => state.counter);

    return (
        <header className="w-full border-b bg-zinc-50 shadow-sm">
            <div className="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
                {/* Navigation Menu */}
                <NavigationMenu>
                    <NavigationMenuList className="gap-6">
                        <NavigationMenuItem>
                            <NavigationMenuLink asChild>
                                <Link
                                    to="/"
                                    className="text-zinc-800 hover:text-blue-600 transition-colors font-semibold text-base"
                                >
                                    Main
                                </Link>
                            </NavigationMenuLink>
                        </NavigationMenuItem>

                        <NavigationMenuItem>
                            <NavigationMenuLink asChild>
                                <Link
                                    to="/settings"
                                    className="text-zinc-800 hover:text-blue-600 transition-colors font-semibold text-base"
                                >
                                    Settings
                                </Link>
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>

                {/* Right side */}
                <div className="flex items-center gap-4">
                    <div className="text-sm text-zinc-700 bg-zinc-200 px-3 py-1 rounded-full">Counter: {counter}</div>
                    <AuthDialog />
                </div>
            </div>
        </header>
    );
}

export default Header;
