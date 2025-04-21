import { AuthDialog } from '@/components/AuthDialog';
import { SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '@/components/ui/sidebar';
import { LogInIcon } from 'lucide-react';
import React, { useState } from 'react';

function NavGuest() {
    const [open, setOpen] = useState(false);

    const handleOpenChange = (open: boolean) => {
        setOpen(open);
    };

    return (
        <React.Fragment>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton
                        size="lg"
                        className="flex gap-3"
                        onClick={() => {
                            setOpen(true);
                        }}
                    >
                        <div className="flex-1">Log in</div>
                        <LogInIcon className="size-4 mr-2" />
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
            <AuthDialog
                open={open}
                onOpenChange={handleOpenChange}
            />
        </React.Fragment>
    );
}

export default NavGuest;
