import { Button } from '@/components/ui/button';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Dialog, DialogContent, DialogDescription } from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Separator } from '@/components/ui/separator';
import api from '@/lib/api';
import { logger } from '@/lib/logger';
import useAuthStore from '@/lib/store/auth';
import { faGoogle } from '@fortawesome/free-brands-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useGoogleLogin } from '@react-oauth/google';
import { useEffect, useState } from 'react';

interface AuthDialogProps {
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
}

export function AuthDialog({ open = false, onOpenChange }: AuthDialogProps) {
    const setAccessToken = useAuthStore((state) => state.setAccessToken);
    const setRefreshToken = useAuthStore((state) => state.setRefreshToken);
    const setUser = useAuthStore((state) => state.setUser);

    const [mode, setMode] = useState<'login' | 'register'>('login');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [confirm, setConfirm] = useState('');
    const [error, setError] = useState('');
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        if (open) {
            setMode('login');
            setEmail('');
            setPassword('');
            setConfirm('');
            setError('');
        }
    }, [open]);

    const login = async (accessToken: string, refreshToken: string) => {
        setAccessToken(accessToken);
        setRefreshToken(refreshToken);

        logger.debug('Stored access token:', accessToken);
        logger.debug('Stored refresh token:', refreshToken);

        // Fetch user data
        const res = await api.post('/auth/me');
        setUser(res.data);

        if (onOpenChange) {
            onOpenChange(false);
        }
    };

    const googleLogin = useGoogleLogin({
        onSuccess: async (tokenResponse) => {
            try {
                const res = await api.post('/auth/google', undefined, {
                    params: {
                        code: tokenResponse.code,
                    },
                });

                const data = res.data;
                await login(data.access_token, data.refresh_token);
            } catch (err) {
                logger.error('Google login verification error:', err);
                setError('Google login failed');
            }
        },
        onError: (err) => {
            logger.error('Google login error:', err);
            setError('Google login failed');
        },
        flow: 'auth-code',
    });

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
            await login(data.access_token, data.refresh_token);
        } catch (err) {
            logger.error('Login/Register error:', err);
            setError('Something went wrong');
        } finally {
            setLoading(false);
        }
    };

    return (
        <Dialog
            open={open}
            onOpenChange={onOpenChange}
        >
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

                        <CardFooter className="pt-4 flex-col gap-4">
                            <div className="w-full flex justify-between">
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
                            </div>
                            <Separator />
                            <Button
                                variant="outline"
                                type="button"
                                className="w-full"
                                onClick={() => {
                                    googleLogin();
                                }}
                            >
                                <FontAwesomeIcon icon={faGoogle} />
                                Continue with Google
                            </Button>
                        </CardFooter>
                    </form>
                </Card>
            </DialogContent>
        </Dialog>
    );
}
