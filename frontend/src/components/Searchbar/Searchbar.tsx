import { Input } from '@/components/ui/input';
import { cn } from '@/lib/utils';
import { SearchIcon } from 'lucide-react';

interface SearchbarProps {
    value?: string;
    onChange?: (value: string) => void;
    onSubmit?: (value: string) => void;
    iconPosition?: 'left' | 'right';
}

function Searchbar({ value = '', onChange, onSubmit, iconPosition = 'right' }: SearchbarProps) {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (onChange) {
            onChange(e.target.value);
        }
    };

    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            if (onSubmit) {
                onSubmit(value);
            }
        }
    };

    return (
        <div className="relative h-fit w-full">
            <Input
                type="text"
                placeholder="Search"
                value={value}
                className={cn(iconPosition === 'right' ? 'pr-10' : 'pl-10')}
                onKeyDown={handleKeyDown}
                onChange={handleChange}
            />
            <SearchIcon
                className={cn(
                    'absolute top-1/2 -translate-y-1/2 text-gray-500 size-4',
                    iconPosition === 'left' ? 'left-3' : 'right-3',
                )}
            />
        </div>
    );
}

export default Searchbar;
