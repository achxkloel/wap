import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Slider } from '@/components/ui/slider';
import { Switch } from '@/components/ui/switch';
import api from '@/lib/api';
import { logger } from '@/lib/logger';
import { useEffect, useState } from 'react';

function Settings() {
    const [theme, setTheme] = useState<'Light' | 'Dark'>('Light');
    const [notificationsEnabled, setNotificationsEnabled] = useState(true);
    const [radius, setRadius] = useState(50);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState('');

    useEffect(() => {
        const loadSettings = async () => {
            try {
                const res = await api.get('/user/settings');
                const data = res.data;
                setTheme(data.theme);
                setNotificationsEnabled(data.notifications_enabled);
                setRadius(data.radius);
            } catch (err) {
                logger.error('Settings load error', err);
                setError('Failed to load settings');
            } finally {
                setLoading(false);
            }
        };

        loadSettings();
    }, []);

    const handleSave = async () => {
        try {
            const body = JSON.stringify({
                theme,
                notifications_enabled: notificationsEnabled,
                radius,
            });

            logger.debug('Settings body', body);
            await api.put('/user/settings', body);
            logger.debug('Settings saved');
        } catch (err) {
            logger.error('Save failed', err);
            setError('Failed to save');
        }
    };

    if (loading) return <div className="p-6 text-center">Loading settings...</div>;

    return (
        <div className="max-w-xl mx-auto p-6">
            <Card>
                <CardHeader>
                    <CardTitle>Settings</CardTitle>
                    <CardDescription>Manage your account preferences</CardDescription>
                </CardHeader>
                <CardContent className="space-y-6">
                    {error && <p className="text-sm text-red-500">{error}</p>}

                    <div className="space-y-2">
                        <Label>Theme</Label>
                        <Select
                            value={theme}
                            onValueChange={(value: 'Light' | 'Dark') => setTheme(value)}
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
                        <div className="text-sm text-gray-500 mt-1">Current: {radius} km</div>
                    </div>
                </CardContent>
                <CardFooter className="flex justify-end">
                    <Button onClick={handleSave}>Save Settings</Button>
                </CardFooter>
            </Card>
        </div>
    );
}

export default Settings;
