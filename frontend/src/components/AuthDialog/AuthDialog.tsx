import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/ui/dialog';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { environment } from '@/environment/environment';
import { logger } from '@/util/utils';
import Cookies from 'js-cookie';

export function AuthDialog() {
    const [mode, setMode] = useState<'login' | 'register'>('login');
    const [isOpen, setIsOpen] = useState(false);
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [confirm, setConfirm] = useState('');
    const [error, setError] = useState('');
    const [loading, setLoading] = useState(false);
    const [isLoggedIn, setIsLoggedIn] = useState(false);
    const navigate = useNavigate();

    useEffect(() => {
        const token = localStorage.getItem('auth_token');
        setIsLoggedIn(!!token);
    }, []);

    const handleOpenChange = (open: boolean) => {
        setIsOpen(open);
        if (open) {
            setMode('login');
            setEmail('');
            setPassword('');
            setConfirm('');
            setError('');
        }
    };

    const handleSubmit = async () => {
        setError('');

        if (mode === 'register' && password !== confirm) {
            setError('Passwords do not match');
            return;
        }

        const endpoint = mode === 'login' ? '/auth/login' : '/auth/register';

        const payload = { email, password };

        try {
            setLoading(true);
            const res = await fetch(environment.baseUrl + endpoint, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload),
            });

            const data = await res.json();

            if (!res.ok) {
                setError(data?.message || 'Something went wrong');
                return;
            }

            const token = data.token;
            if (token) {
                localStorage.setItem('auth_token', token);
                setIsLoggedIn(true);
                logger.debug('Token stored:', token);
            }

            setIsOpen(false);
        } catch (err) {
            console.error(err);
            setError('Something went wrong');
        } finally {
            setLoading(false);
        }

        const token = Cookies.get('token');
        logger.debug('Token from cookie:', token);
    };

    const handleLogout = async () => {
        try {
            const res = await fetch(`${environment.baseUrl}/auth/logout`, {
                method: 'POST',
            });

            if (!res.ok) throw new Error('Logout failed');

            localStorage.removeItem('auth_token');
            setIsLoggedIn(false);
            logger.debug('User logged out');
            navigate('/');
        } catch (err) {
            logger.error('Logout error', err);
        }
    };

    // ✅ If user is logged in, show logout button
    if (isLoggedIn) {
        return (
            <Button
                variant="outline"
                onClick={handleLogout}
            >
                Logout
            </Button>
        );
    }

    // ✅ Otherwise show auth dialog
    return (
        <Dialog
            open={isOpen}
            onOpenChange={handleOpenChange}
        >
            <DialogTrigger asChild>
                <Button variant="outline">Login</Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[450px]">
                <Card className="w-full shadow-none border-0">
                    <CardHeader className="pb-2">
                        <CardTitle className="text-2xl font-semibold">
                            {mode === 'login' ? 'Login' : 'Register'}
                        </CardTitle>
                        <DialogDescription>
                            {mode === 'login'
                                ? 'Login to access your account.'
                                : 'Create a new account to get started.'}
                        </DialogDescription>
                    </CardHeader>

                    <form
                        onSubmit={(e) => {
                            e.preventDefault();
                            handleSubmit();
                        }}
                        onKeyDown={(e) => {
                            if (e.key === 'Enter') {
                                e.preventDefault();
                                handleSubmit();
                            }
                        }}
                    >
                        <CardContent className="space-y-4">
                            <div className="grid gap-2">
                                <Label htmlFor="email">Email</Label>
                                <Input
                                    id="email"
                                    type="email"
                                    value={email}
                                    onChange={(e) => setEmail(e.target.value)}
                                    required
                                />
                            </div>
                            <div className="grid gap-2">
                                <Label htmlFor="password">Password</Label>
                                <Input
                                    id="password"
                                    type="password"
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                    required
                                />
                            </div>
                            {mode === 'register' && (
                                <div className="grid gap-2">
                                    <Label htmlFor="confirm">Confirm Password</Label>
                                    <Input
                                        id="confirm"
                                        type="password"
                                        value={confirm}
                                        onChange={(e) => setConfirm(e.target.value)}
                                        required
                                    />
                                </div>
                            )}
                            {error && <p className="text-sm text-red-500">{error}</p>}
                        </CardContent>

                        <CardFooter className="flex justify-between pt-4">
                            <Button
                                variant="ghost"
                                type="button"
                                onClick={() => setMode(mode === 'login' ? 'register' : 'login')}
                            >
                                {mode === 'login' ? 'Switch to Register' : 'Switch to Login'}
                            </Button>
                            <Button
                                type="submit"
                                disabled={loading}
                            >
                                {loading ? 'Loading...' : mode === 'login' ? 'Login' : 'Register'}
                            </Button>
                        </CardFooter>
                    </form>
                </Card>
            </DialogContent>
        </Dialog>
    );
}
