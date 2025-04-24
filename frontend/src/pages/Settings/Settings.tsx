import { useEffect, useRef, useState } from 'react';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Slider } from '@/components/ui/slider';
import { Switch } from '@/components/ui/switch';
import { Input } from '@/components/ui/input';
import { useToast } from '@/hooks/use-toast';
import { Eye, EyeOff } from 'lucide-react';
import api from '@/lib/api';
import { logger } from '@/lib/logger';
import { z } from 'zod';

/* ------------------------------- validation -------------------------------- */
const passwordSchema = z
    .object({
        currentPassword: z.string().min(1, 'Current password is required'),
        newPassword: z.string().min(8, 'New password must be at least 8 characters'),
        confirmPassword: z.string().min(1, 'Please confirm the new password'),
    })
    .refine((v) => v.newPassword === v.confirmPassword, {
        path: ['confirmPassword'],
        message: 'Passwords do not match',
    });

/* --------------------------- helper: error text ---------------------------- */
const getErrorMessage = (err: unknown, fallback: string) => {
    // Axios-like error shape
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const maybe = err as any;
    return maybe?.response?.data?.data || maybe?.message || fallback;
};

/* -------------------------------------------------------------------------- */
export default function Settings() {
    const { toast } = useToast();

    /* ------------------------------- settings -------------------------------- */
    const [theme, setTheme] = useState<'Light' | 'Dark'>('Light');
    const [notificationsEnabled, setNotificationsEnabled] = useState(true);
    const [radius, setRadius] = useState(50);
    const [loading, setLoading] = useState(true);

    /* ----------------------------- button state ------------------------------ */
    type BtnState = 'idle' | 'loading' | 'saved';
    const [prefBtnState, setPrefBtnState] = useState<BtnState>('idle');
    const [pwBtnState, setPwBtnState] = useState<BtnState>('idle');
    const savedTimerRef = useRef<NodeJS.Timeout | null>(null);
    const pwSavedTimerRef = useRef<NodeJS.Timeout | null>(null);

    /* ---------------------------- password form ------------------------------ */
    const [pwFields, setPwFields] = useState({
        currentPassword: '',
        newPassword: '',
        confirmPassword: '',
    });
    const [pwVisible, setPwVisible] = useState({
        currentPassword: false,
        newPassword: false,
        confirmPassword: false,
    });
    const [pwErrors, setPwErrors] = useState<Record<string, string>>({});

    /* ------------------------------ load data ------------------------------- */
    useEffect(() => {
        (async () => {
            try {
                const { data } = await api.get('/user/settings');
                setTheme(data.theme);
                setNotificationsEnabled(data.notifications_enabled);
                setRadius(data.radius);
                // toast({
                //     variant: "destructive",
                //     description: 'Settings loaded',
                // });
            } catch (err) {
                logger.error('Settings load error', err);
                toast({
                    variant: 'destructive',
                    description: getErrorMessage(err, 'Failed to load settings'),
                });
            } finally {
                setLoading(false);
            }
        })();
    }, [toast]);

    /* ----------------------------- save prefs -------------------------------- */
    const handleSavePrefs = async () => {
        setPrefBtnState('loading');
        try {
            await api.put('/user/settings', {
                theme,
                notifications_enabled: notificationsEnabled,
                radius,
            });

            toast({
                variant: 'destructive',
                description: 'Settings saved successfully',
            });

            setPrefBtnState('saved');
            if (savedTimerRef.current) clearTimeout(savedTimerRef.current);
            savedTimerRef.current = setTimeout(() => setPrefBtnState('idle'), 1000);
        } catch (err) {
            logger.error('Save failed', err);
            toast({
                variant: 'destructive',
                description: getErrorMessage(err, 'Failed to save settings'),
            });
            setPrefBtnState('idle');
        }
    };

    /* --------------------------- change password ----------------------------- */
    const handlePasswordChange = async () => {
        const parse = passwordSchema.safeParse(pwFields);
        if (!parse.success) {
            setPwErrors(
                Object.fromEntries(
                    Object.entries(parse.error.flatten().fieldErrors).map(([k, v]) => [k, v?.[0] ?? '']),
                ),
            );
            return;
        }

        setPwErrors({});
        setPwBtnState('loading');
        try {
            await api.post('/auth/change-password', {
                current_password: pwFields.currentPassword,
                new_password: pwFields.newPassword,
            });

            toast({
                variant: 'destructive',
                description: 'Password changed successfully',
            });

            setPwFields({ currentPassword: '', newPassword: '', confirmPassword: '' });
            setPwBtnState('saved');
            if (pwSavedTimerRef.current) clearTimeout(pwSavedTimerRef.current);
            pwSavedTimerRef.current = setTimeout(() => setPwBtnState('idle'), 1000);
        } catch (err) {
            logger.error('Password change failed', err);
            toast({
                variant: 'destructive',
                description: getErrorMessage(err, 'Password change failed'),
            });
            setPwBtnState('idle');
        }
    };

    /* ------------------------------ cleanup ---------------------------------- */
    useEffect(
        () => () => {
            if (savedTimerRef.current) clearTimeout(savedTimerRef.current);
            if (pwSavedTimerRef.current) clearTimeout(pwSavedTimerRef.current);
        },
        [],
    );

    /* --------------------------- helper renders ------------------------------ */
    const renderPasswordInput = (id: keyof typeof pwFields, label: string) => (
        <div className="space-y-2">
            <Label htmlFor={id}>{label}</Label>
            <div className="relative">
                <Input
                    id={id}
                    type={pwVisible[id] ? 'text' : 'password'}
                    value={pwFields[id]}
                    onChange={(e) => setPwFields((s) => ({ ...s, [id]: e.target.value }))}
                />
                <Button
                    type="button"
                    size="icon"
                    variant="ghost"
                    className="absolute right-1 top-1/2 -translate-y-1/2"
                    onClick={() => setPwVisible((v) => ({ ...v, [id]: !v[id] }))}
                    aria-label={pwVisible[id] ? 'Hide password' : 'Show password'}
                >
                    {pwVisible[id] ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
                </Button>
            </div>
            {pwErrors[id] && <p className="text-sm text-red-500">{pwErrors[id]}</p>}
        </div>
    );

    /* -------------------------------- render --------------------------------- */
    if (loading) return <div className="p-6 text-center">Loading settings…</div>;

    return (
        <div className="min-w-96 mx-auto p-8 space-y-8">
            {/* Preferences --------------------------------------------------------- */}
            <Card>
                <CardHeader>
                    <CardTitle>Preferences</CardTitle>
                    <CardDescription>Manage your account preferences</CardDescription>
                </CardHeader>
                <CardContent className="space-y-6">
                    <div className="space-y-2">
                        <Label>Theme</Label>
                        <Select
                            value={theme}
                            onValueChange={(v: 'Light' | 'Dark') => setTheme(v)}
                        >
                            <SelectTrigger>
                                <SelectValue placeholder="Select theme" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="Light">Light</SelectItem>
                                <SelectItem value="Dark">Dark</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>

                    <div className="flex items-center justify-between">
                        <Label>Notifications</Label>
                        <Switch
                            checked={notificationsEnabled}
                            onCheckedChange={setNotificationsEnabled}
                        />
                    </div>

                    <div>
                        <Label className="mb-1 block">Radius (km)</Label>
                        <Slider
                            min={0}
                            max={100}
                            step={1}
                            value={[radius]}
                            onValueChange={([val]) => setRadius(val)}
                        />
                        <div className="text-sm text-muted-foreground mt-1">Current: {radius} km</div>
                    </div>
                </CardContent>
                <CardFooter className="flex justify-end">
                    <Button
                        onClick={handleSavePrefs}
                        disabled={prefBtnState === 'loading'}
                    >
                        {prefBtnState === 'loading' ? 'Saving…' : prefBtnState === 'saved' ? 'Saved' : 'Save changes'}
                    </Button>
                </CardFooter>
            </Card>

            {/* Password ------------------------------------------------------------ */}
            <Card>
                <CardHeader>
                    <CardTitle>Password</CardTitle>
                    <CardDescription>Update your account password</CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                    {renderPasswordInput('currentPassword', 'Current password')}
                    {renderPasswordInput('newPassword', 'New password')}
                    {renderPasswordInput('confirmPassword', 'Confirm new password')}
                </CardContent>
                <CardFooter className="flex justify-end">
                    <Button
                        onClick={handlePasswordChange}
                        disabled={pwBtnState === 'loading'}
                    >
                        {pwBtnState === 'loading' ? 'Changing…' : pwBtnState === 'saved' ? 'Saved' : 'Change password'}
                    </Button>
                </CardFooter>
            </Card>
        </div>
    );
}
