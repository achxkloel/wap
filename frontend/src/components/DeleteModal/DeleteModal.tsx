import { Button } from '@/components/ui/button';
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog';

interface DeleteModalProps {
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
    onDelete?: () => void;
    title?: string;
    description?: string;
}

function DeleteModal({
    open,
    onOpenChange,
    onDelete,
    title = '',
    description = 'Are you sure you want to delete this item?',
}: DeleteModalProps) {
    return (
        <Dialog
            open={open}
            onOpenChange={onOpenChange}
        >
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>{title}</DialogTitle>
                    <DialogDescription>{description}</DialogDescription>
                </DialogHeader>
                <DialogFooter>
                    <Button
                        type="button"
                        variant="outline"
                        onClick={() => {
                            if (onOpenChange) {
                                onOpenChange(false);
                            }
                        }}
                    >
                        Cancel
                    </Button>
                    <Button
                        type="button"
                        variant="destructive"
                        onClick={() => {
                            if (onDelete) {
                                onDelete();
                            }
                        }}
                    >
                        Delete
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    );
}

export default DeleteModal;
