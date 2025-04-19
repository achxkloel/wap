import { Input } from '@/components/ui/input';
import { cn } from '@/lib/utils';
import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons/faMagnifyingGlass';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useState } from 'react';

interface SearchbarProps {
    onChange?: (value: string) => void;
    onSubmit?: (value: string) => void;
    iconPosition?: 'left' | 'right';
}

function Searchbar({ onChange, onSubmit, iconPosition = 'right' }: SearchbarProps) {
    const [value, setValue] = useState('');

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setValue(e.target.value);
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
                className={cn(iconPosition === 'right' ? 'pr-10' : 'pl-10')}
                onKeyDown={handleKeyDown}
                onChange={handleChange}
            />
            <FontAwesomeIcon
                icon={faMagnifyingGlass}
                className={cn(
                    'absolute top-1/2 -translate-y-1/2 text-gray-500',
                    iconPosition === 'left' ? 'left-3' : 'right-3',
                )}
            />
        </div>
    );
}
export default Searchbar;
