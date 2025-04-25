import { Button } from '@/components/ui/button';
import { PencilIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';

interface LocationCardProps {
    location: any;
    onEdit?: (location: any) => void;
    onDelete?: (location: any) => void;
}

function LocationCard({ location, onEdit, onDelete }: LocationCardProps) {
    const [hover, setHover] = useState(false);

    return (
        <div
            className="flex gap-4 border rounded-lg shadow-md p-4 hover:shadow-lg transition-shadow duration-200 ease-in-out bg-sidebar"
            onMouseEnter={() => setHover(true)}
            onMouseLeave={() => setHover(false)}
        >
            <div className="flex-shrink-0">
                <img
                    src={location.photo ?? undefined}
                    alt={location.name}
                    className="w-[200px] h-[200px] rounded-md object-cover"
                />
            </div>
            <div className="flex-1 flex flex-col gap-4 justify-between">
                <div className="flex flex-col">
                    <div className="flex justify-between items-start">
                        <h2 className="text-2xl font-bold pb-1">{location.name}</h2>
                        {hover && (
                            <div className="flex gap-2">
                                <Button
                                    variant="outline"
                                    size="icon"
                                    className="p-2 dark:hover:bg-black"
                                    onClick={() => {
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
                                    onClick={() => {
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
                    <p className="text-gray-500 dark:text-gray-50">{location.description}</p>
                </div>
                <div className="grid grid-cols-2">
                    <div>
                        <div>Coordinates</div>
                        <div>Radius</div>
                        <div>Weather</div>
                        <div>Earthquakes</div>
                    </div>

                    <div>
                        <div>
                            {location.latitude}, {location.longitude}
                        </div>
                        <div>{location.radius} km</div>
                        <div></div>
                        <div></div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default LocationCard;
