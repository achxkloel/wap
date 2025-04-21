import useData from '@/lib/store/data';
import { format } from 'date-fns';
import { Virtuoso } from 'react-virtuoso';

interface EventListProps {
    search?: string;
}

function EventList({ search }: EventListProps) {
    const earthquakes = useData((state) => state.earthquake);

    if (!earthquakes || earthquakes.features.length <= 0) {
        return <div className="p-4 text-center text-gray-500 h-18 ">No items found</div>;
    }

    const getEarthquakes = () => {
        if (!earthquakes) {
            return;
        }

        if (!search) {
            return earthquakes.features;
        }

        return earthquakes.features.filter((item) => {
            const place = item.properties.place.toLowerCase();
            return place.includes(search.toLowerCase());
        });
    };

    return (
        <div className="h-full">
            <Virtuoso
                style={{ height: '100%' }}
                data={getEarthquakes()}
                itemContent={(index, item) => (
                    <div
                        key={index}
                        className="p-3 cursor-pointer border-b h-18 hover:bg-gray-50"
                    >
                        <h4 className="text-md font-semibold text-ellipsis overflow-hidden whitespace-nowrap">
                            {item.properties.place}
                        </h4>
                        <p className="text-sm text-gray-500 text-ellipsis overflow-hidden whitespace-nowrap">
                            {format(item.properties.time, 'PPPpp')}
                        </p>
                    </div>
                )}
            />
        </div>
    );
}

export default EventList;
