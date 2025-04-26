import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import { logger } from '@/lib/logger';
import useAuthStore from '@/lib/store/auth';
import { LogOutIcon } from 'lucide-react';
import { useNavigate } from 'react-router';

function NavUser() {
    const removeAccessToken = useAuthStore((state) => state.removeAccessToken);
    const removeRefreshToken = useAuthStore((state) => state.removeRefreshToken);
    const user = useAuthStore((state) => state.user);
    const setUser = useAuthStore((state) => state.setUser);
    const navigate = useNavigate();

    const handleLogout = async () => {
        try {
            removeAccessToken();
            removeRefreshToken();
            setUser(null);
            logger.debug('User logged out');
            navigate('/');
        } catch (err) {
            logger.error('Logout error', err);
        }
    };

    const getUserInitials = () => {
        if (!user) {
            return '';
        }

        if (!user.firstName && !user.lastName) {
            return user.email.slice(0, 2).toUpperCase();
        }

        const firstInitial = (user.firstName || '').charAt(0).toUpperCase();
        const lastInitial = (user.lastName || '').charAt(0).toUpperCase();

        return `${firstInitial}${lastInitial}`;
    };

    const getUserFullName = () => {
        if (!user) {
            return '';
        }

        if (!user.firstName && !user.lastName) {
            return '';
        }

        const firstName = user.firstName || '';
        const lastName = user.lastName || '';

        return `${firstName} ${lastName}`;
    };

    const fullName = getUserFullName();
    const initials = getUserInitials();

    return (
        <div className="flex w-full items-center px-2 py-2 gap-3 rounded-lg">
            <Avatar className="h-8 w-8 rounded-lg">
                <AvatarImage
                    src={user?.imageUrl ?? undefined}
                    alt={fullName}
                />
                <AvatarFallback className="rounded-lg">{initials}</AvatarFallback>
            </Avatar>
            <div className="grid flex-1 text-left text-sm leading-tight">
                {fullName && <span className="truncate font-medium">{fullName}</span>}
                <span className="truncate text-xs text-muted-foreground">{user?.email || ''}</span>
            </div>
            <Button
                variant="ghost"
                className="p-2"
                onClick={handleLogout}
            >
                <LogOutIcon className="size-4" />
            </Button>
        </div>
    );
}

export default NavUser;
