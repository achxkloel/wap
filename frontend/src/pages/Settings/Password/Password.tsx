import { Button } from '@/components/ui/button';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { toast } from '@/hooks/use-toast';
import api from '@/lib/api';
import { logger } from '@/lib/logger';
import { getErrorMessage } from '@/lib/utils';
import { zodResolver } from '@hookform/resolvers/zod';
import { EyeIcon, EyeOffIcon } from 'lucide-react';
import React, { useEffect, useState } from 'react';
import { ControllerRenderProps, useForm } from 'react-hook-form';
import { z } from 'zod';

const formSchema = z
    .object({
        currentPassword: z.string().min(1, 'Current password is required'),
        newPassword: z.string().min(8, 'New password must be at least 8 characters long'),
        confirmPassword: z.string().min(1, 'Please confirm the new password'),
    })
    .refine((v) => v.newPassword === v.confirmPassword, {
        path: ['confirmPassword'],
        message: 'Passwords do not match',
    });

type PasswordFormValues = z.infer<typeof formSchema>;

function Password() {
    const form = useForm<PasswordFormValues>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            currentPassword: '',
            newPassword: '',
            confirmPassword: '',
        },
    });

    const [pwVisible, setPwVisible] = useState({
        currentPassword: false,
        newPassword: false,
        confirmPassword: false,
    });

    type BtnState = 'idle' | 'loading' | 'saved';
    const [pwBtnState, setPwBtnState] = useState<BtnState>('idle');
    let pwSavedTimer: ReturnType<typeof setTimeout> | null = null;

    useEffect(() => {
        return () => {
            if (pwSavedTimer) clearTimeout(pwSavedTimer);
        };
    }, []);

    const onSubmit = async (data: PasswordFormValues) => {
        setPwBtnState('loading');

        try {
            await api.post('/auth/change-password', {
                current_password: data.currentPassword,
                new_password: data.newPassword,
            });

            toast({
                description: 'Password changed successfully',
            });

            form.reset();
            setPwBtnState('saved');
            if (pwSavedTimer) clearTimeout(pwSavedTimer);
            pwSavedTimer = setTimeout(() => setPwBtnState('idle'), 1000);
        } catch (err) {
            logger.error('Password change failed', err);
            toast({
                variant: 'destructive',
                description: getErrorMessage(err, 'Password change failed'),
            });
            setPwBtnState('idle');
        }
    };

    const renderPasswordField = (field: ControllerRenderProps<PasswordFormValues>, label: string) => {
        const isVisible = pwVisible[field.name];

        return (
            <FormItem>
                <React.Fragment>
                    <FormLabel>{label}</FormLabel>
                    <FormControl>
                        <div className="relative">
                            <Input
                                {...field}
                                type={isVisible ? 'text' : 'password'}
                            />
                            <Button
                                type="button"
                                size="icon"
                                variant="ghost"
                                className="absolute right-1 top-1/2 -translate-y-1/2"
                                onClick={() => setPwVisible((prev) => ({ ...prev, [field.name]: !prev[field.name] }))}
                                aria-label={isVisible ? 'Hide password' : 'Show password'}
                            >
                                {isVisible ? <EyeOffIcon className="h-4 w-4" /> : <EyeIcon className="h-4 w-4" />}
                            </Button>
                        </div>
                    </FormControl>
                    <FormMessage />
                </React.Fragment>
            </FormItem>
        );
    };

    return (
        <Form {...form}>
            <form
                onSubmit={form.handleSubmit(onSubmit)}
                noValidate
                autoComplete="off"
            >
                <div className="space-y-4">
                    <FormField
                        control={form.control}
                        name="currentPassword"
                        render={({ field }) => renderPasswordField(field, 'Current password')}
                    />
                    <FormField
                        control={form.control}
                        name="newPassword"
                        render={({ field }) => renderPasswordField(field, 'New password')}
                    />
                    <FormField
                        control={form.control}
                        name="confirmPassword"
                        render={({ field }) => renderPasswordField(field, 'Confirm new password')}
                    />
                    <div className="pt-4">
                        <Button
                            type="submit"
                            disabled={pwBtnState === 'loading'}
                        >
                            {pwBtnState === 'loading'
                                ? 'Changing…'
                                : pwBtnState === 'saved'
                                  ? 'Saved'
                                  : 'Change password'}
                        </Button>
                    </div>
                </div>
            </form>
        </Form>
    );
}

export default Password;
