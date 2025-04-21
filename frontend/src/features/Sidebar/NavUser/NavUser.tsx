import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import api from '@/lib/api';
import { logger } from '@/lib/logger';
import useAuthStore from '@/lib/store/auth';
import { LogOutIcon } from 'lucide-react';
import { useNavigate } from 'react-router';

function NavUser() {
    const removeAccessToken = useAuthStore((state) => state.removeAccessToken);
    const removeRefreshToken = useAuthStore((state) => state.removeRefreshToken);
    const navigate = useNavigate();

    const handleLogout = async () => {
        try {
            await api.post('/auth/logout');
            removeAccessToken();
            removeRefreshToken();
            logger.debug('User logged out');
            navigate('/');
        } catch (err) {
            logger.error('Logout error', err);
        }
    };

    return (
        <div className="flex w-full items-center px-2 py-2 gap-3 rounded-lg">
            <Avatar className="h-8 w-8 rounded-lg grayscale">
                <AvatarImage
                    src={''}
                    alt="Jan Novak"
                />
                <AvatarFallback className="rounded-lg">JN</AvatarFallback>
            </Avatar>
            <div className="grid flex-1 text-left text-sm leading-tight">
                <span className="truncate font-medium">Jan Novak</span>
                <span className="truncate text-xs text-muted-foreground">jan_novak@gmail.com</span>
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
