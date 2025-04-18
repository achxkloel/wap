import { Input } from '@/components/ui/input';
import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons/faMagnifyingGlass';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useState } from 'react';

interface SearchbarProps {
    onChange?: (value: string) => void;
    onSubmit?: (value: string) => void;
}

function Searchbar({ onChange, onSubmit }: SearchbarProps) {
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
        <div className="relative">
            <Input
                type="text"
                placeholder="Search"
                className="w-full pr-10"
                onKeyDown={handleKeyDown}
                onChange={handleChange}
            />
            <FontAwesomeIcon
                icon={faMagnifyingGlass}
                className="absolute top-1/2 right-3 -translate-y-1/2 text-gray-500"
            />
        </div>
    );
}
export default Searchbar;
