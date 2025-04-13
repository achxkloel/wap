import { Button } from '@/components/ui/button';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Dialog, DialogContent, DialogDescription, DialogTrigger } from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import api from '@/lib/api';
import { logger } from '@/lib/logger';
import useAuthStore from '@/lib/store/auth';
import Cookies from 'js-cookie';
import { useState } from 'react';
import { useNavigate } from 'react-router';

export function AuthDialog() {
    const setToken = useAuthStore((state) => state.setToken);
    const removeToken = useAuthStore((state) => state.removeToken);
    const token = useAuthStore((state) => state.token);

    const [mode, setMode] = useState<'login' | 'register'>('login');
    const [isOpen, setIsOpen] = useState(false);
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [confirm, setConfirm] = useState('');
    const [error, setError] = useState('');
    const [loading, setLoading] = useState(false);
    const navigate = useNavigate();

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
            const res = await api.post(endpoint, payload);
            const data = res.data;
            const token = data.token;

            if (token) {
                setToken(token);
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
            await api.post('/auth/logout');
            removeToken();
            logger.debug('User logged out');
            navigate('/');
        } catch (err) {
            logger.error('Logout error', err);
        }
    };

    // ✅ If user is logged in, show logout button
    if (token !== null) {
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
