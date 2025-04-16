import { AuthDialog } from '@/components/AuthDialog';
import { useIsAuthorized } from '@/lib/store/auth';
import { IconDefinition } from '@fortawesome/fontawesome-svg-core';
import { faCloud } from '@fortawesome/free-solid-svg-icons/faCloud';
import { faGear } from '@fortawesome/free-solid-svg-icons/faGear';
import { faHouse } from '@fortawesome/free-solid-svg-icons/faHouse';
import { faMap } from '@fortawesome/free-solid-svg-icons/faMap';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import clsx from 'clsx';
import { Link, useLocation } from 'react-router';

interface NavItem {
    label: string;
    to: string;
    icon: IconDefinition;
    protected?: boolean;
}

const navItems: NavItem[] = [
    { label: 'Weather', to: '/', icon: faHouse },
    { label: 'Locations', to: '/locations', icon: faCloud },
    { label: 'Map', to: '/map', icon: faMap },
    { label: 'Settings', to: '/settings', icon: faGear, protected: true },
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
        <div className="w-64 shadow-md p-4 flex flex-col space-y-4">
            <nav className="flex-1 flex flex-col space-y-2">
                {navItems.map((item, index) => {
                    if (item.protected && !isAuthorized) {
                        return null;
                    }

                    return (
                        <div
                            key={index}
                            className={clsx(
                                'px-3 py-2 rounded transition flex items-center space-x-4 cursor-pointer',
                                isSelected(item.to) ? 'bg-gray-200' : 'text-gray-700 hover:bg-gray-200',
                            )}
                        >
                            <FontAwesomeIcon icon={item.icon}></FontAwesomeIcon>
                            <Link
                                to={item.to}
                                className="flex-1"
                            >
                                {item.label}
                            </Link>
                        </div>
                    );
                })}
            </nav>
            <AuthDialog />
        </div>
    );
}

export default Sidebar;
