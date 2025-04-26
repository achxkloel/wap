import { Button } from '@/components/ui/button';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { toast } from '@/hooks/use-toast';
import api from '@/lib/api';
import { logger } from '@/lib/logger';
import useAuthStore from '@/lib/store/auth';
import { getErrorMessage } from '@/lib/utils';
import { zodResolver } from '@hookform/resolvers/zod';
import { format } from 'date-fns';
import { CheckIcon, XIcon } from 'lucide-react';
import { useEffect, useState } from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

const formSchema = z.object({
    firstName: z.string(),
    lastName: z.string(),
});

type ProfileFormValues = z.infer<typeof formSchema>;

function Profile() {
    const form = useForm<ProfileFormValues>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            firstName: '',
            lastName: '',
        },
    });

    type BtnState = 'idle' | 'loading' | 'saved';
    const [saveBtnState, setSaveBtnState] = useState<BtnState>('idle');
    let savedTimer: ReturnType<typeof setTimeout> | null = null;
    const user = useAuthStore((state) => state.user);
    const setUser = useAuthStore((state) => state.setUser);
    const isGoogleUser = user?.provider === 'google';

    useEffect(() => {
        if (user) {
            form.setValue('firstName', user.firstName ?? '');
            form.setValue('lastName', user.lastName ?? '');
        }
    }, [user]);

    const onSubmit = async (data: ProfileFormValues) => {
        if (!user || isGoogleUser) {
            return;
        }

        setSaveBtnState('loading');

        try {
            await api.post(`/auth/update-user-info/${user.id}`, {
                first_name: data.firstName,
                last_name: data.lastName,
            });

            toast({
                description: 'Profile updated successfully',
            });

            const res = await api.post('/auth/me');
            setUser(res.data);

            setSaveBtnState('saved');
            if (savedTimer) clearTimeout(savedTimer);
            savedTimer = setTimeout(() => setSaveBtnState('idle'), 1000);
        } catch (err) {
            logger.error('Profile update failed', err);
            toast({
                variant: 'destructive',
                description: getErrorMessage(err, 'Profile update failed'),
            });
            setSaveBtnState('idle');
        }
    };

    return (
        <Form {...form}>
            <form
                onSubmit={form.handleSubmit(onSubmit)}
                noValidate
                autoComplete="off"
            >
                <div className="space-y-4">
                    <FormItem>
                        <FormLabel>Email</FormLabel>
                        <FormControl>
                            <Input
                                readOnly={true}
                                value={user?.email ?? ''}
                            />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                    <FormField
                        control={form.control}
                        name="firstName"
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>First name</FormLabel>
                                <FormControl>
                                    <Input
                                        {...field}
                                        readOnly={isGoogleUser}
                                    />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <FormField
                        control={form.control}
                        name="lastName"
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>Last name</FormLabel>
                                <FormControl>
                                    <Input
                                        {...field}
                                        readOnly={isGoogleUser}
                                    />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <div>
                        <div className="flex justify-between items-center">
                            <div className="text-md font-semibold">Google</div>
                            {isGoogleUser ? <CheckIcon color="#17c400" /> : <XIcon color="#ff3333" />}
                        </div>
                        <div className="flex justify-between items-center">
                            <div className="text-md font-semibold">Created</div>
                            <div className="text-md">{user ? format(user.createdAt, 'PPPpp') : ''}</div>
                        </div>
                        <div className="flex justify-between items-center">
                            <div className="text-md font-semibold">Updated</div>
                            <div className="text-md">{user ? format(user.updatedAt, 'PPPpp') : ''}</div>
                        </div>
                    </div>
                    {!isGoogleUser && (
                        <div className="pt-4">
                            <Button
                                type="submit"
                                disabled={saveBtnState === 'loading'}
                            >
                                {saveBtnState === 'loading'
                                    ? 'Updating…'
                                    : saveBtnState === 'saved'
                                      ? 'Saved'
                                      : 'Save changes'}
                            </Button>
                        </div>
                    )}
                </div>
            </form>
        </Form>
    );
}

export default Profile;
