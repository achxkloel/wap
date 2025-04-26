import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import useAuthStore from '@/lib/store/auth';
import { cn } from '@/lib/utils';
import { useState } from 'react';
import Password from './Password';
import Profile from './Profile';

interface Tab {
    key: 'profile' | 'password';
    label: string;
}

const tabs: Tab[] = [
    { key: 'profile', label: 'Profile' },
    { key: 'password', label: 'Password' },
];

export default function Settings() {
    const user = useAuthStore((state) => state.user);
    const [selectedTab, setSelectedTab] = useState<'profile' | 'password'>('profile');

    return (
        <div className="container mx-auto mt-8 flex flex-wrap gap-4 justify-center">
            <Card className="min-w-full sm:min-w-[500px] bg-sidebar">
                <CardContent className="pt-6">
                    {selectedTab === 'profile' && <Profile />}
                    {selectedTab === 'password' && <Password />}
                </CardContent>
            </Card>
            <Card className="min-w-full sm:min-w-[300px] h-fit bg-sidebar">
                <CardContent className="pt-6">
                    <div className="flex flex-col space-y-1">
                        {tabs.map((tab, index) => {
                            if (tab.key === 'password' && user?.provider === 'google') {
                                return null;
                            }

                            return (
                                <Button
                                    key={index}
                                    variant="ghost"
                                    className={cn('justify-start', selectedTab === tab.key ? 'bg-muted' : '')}
                                    onClick={() => setSelectedTab(tab.key)}
                                >
                                    {tab.label}
                                </Button>
                            );
                        })}
                    </div>
                </CardContent>
            </Card>
        </div>
    );
}
