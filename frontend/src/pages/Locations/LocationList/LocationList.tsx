import defaultLocationImage from '@/assets/default_location.png';
import { Button } from '@/components/ui/button';
import useLocationStore from '@/lib/store/location';
import { cn } from '@/lib/utils';
import { PencilIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import { Virtuoso } from 'react-virtuoso';

interface LocationListProps {
    selected?: any;
    onChange?: (location: any) => void;
    onEdit?: (location: any) => void;
    onDelete?: (location: any) => void;
}

function LocationList({ selected, onChange, onEdit, onDelete }: LocationListProps) {
    const [hover, setHover] = useState<number | null>(null);
    const locations = useLocationStore((state) => state.locations);

    if (!locations || locations.length <= 0) {
        return <div className="p-4 text-center text-gray-500 dark:text-gray-50 h-18 ">No locations found</div>;
    }

    return (
        <div className="h-full">
            <Virtuoso
                style={{ height: '100%' }}
                data={locations}
                itemContent={(index, location) => (
                    <div
                        key={index}
                        onClick={() => {
                            if (onChange) {
                                onChange(location);
                            }
                        }}
                        className={cn(
                            'flex cursor-pointer border-b p-3 gap-4',
                            selected && selected.id === location.id
                                ? 'bg-blue-100 dark:bg-sidebar-primary'
                                : 'hover:bg-muted',
                        )}
                        onMouseEnter={() => setHover(location.id)}
                        onMouseLeave={() => setHover(null)}
                    >
                        <div className="flex-shrink-0">
                            <img
                                src={location.image || defaultLocationImage}
                                alt={location.name}
                                className="w-16 h-16 rounded-md object-cover"
                            />
                        </div>
                        <div className="flex-1 flex flex-col justify-between overflow-hidden">
                            <div className="flex items-start gap-2 justify-between">
                                <h2 className="text-md font-semibold text-ellipsis overflow-hidden whitespace-nowrap">
                                    {location.name}
                                </h2>
                                {hover == location.id && (
                                    <div className="flex gap-2">
                                        <Button
                                            variant="outline"
                                            size="icon"
                                            className="p-2 dark:hover:bg-black"
                                            onClick={(e) => {
                                                e.stopPropagation();

                                                if (onEdit) {
                                                    onEdit(location);
                                                }
                                            }}
                                        >
                                            <PencilIcon className="size-4" />
                                        </Button>
                                        <Button
                                            variant="destructive"
                                            size="icon"
                                            className="p-2"
                                            onClick={(e) => {
                                                e.stopPropagation();

                                                if (onDelete) {
                                                    onDelete(location);
                                                }
                                            }}
                                        >
                                            <Trash2Icon className="size-4" />
                                        </Button>
                                    </div>
                                )}
                            </div>
                            <p className="text-sm text-gray-500 dark:text-gray-50 text-ellipsis overflow-hidden whitespace-nowrap">
                                {location.description}
                            </p>
                        </div>
                    </div>
                )}
            />
        </div>
    );
}

export default LocationList;
