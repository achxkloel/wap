import useEarthquake from '@/lib/store/earthquake';
import { cn } from '@/lib/utils';
import { format } from 'date-fns';
import { useEffect, useRef, useState } from 'react';
import { ListRange, Virtuoso, VirtuosoHandle } from 'react-virtuoso';

interface EventListProps {
    search?: string;
}

function EventList({ search }: EventListProps) {
    const earthquakes = useEarthquake((state) => state.earthquakes);
    const setSelected = useEarthquake((state) => state.setSelected);
    const selected = useEarthquake((state) => state.selected);
    const virtuoso = useRef<VirtuosoHandle>(null);
    const [visibleRange, setVisibleRange] = useState<ListRange>({ startIndex: 0, endIndex: 0 });

    useEffect(() => {
        if (typeof selected === 'undefined') {
            return;
        }

        if (!earthquakes) {
            return;
        }

        const earthquakeIndex = earthquakes.features.findIndex((feature) => feature.id === selected);

        if (earthquakeIndex < 0) {
            return;
        }

        if (!virtuoso.current) {
            return;
        }

        if (earthquakeIndex >= visibleRange.startIndex && earthquakeIndex <= visibleRange.endIndex) {
            return;
        }

        virtuoso.current.scrollToIndex({
            index: earthquakeIndex,
        });
    }, [selected]);

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

    if (!earthquakes || earthquakes.features.length <= 0) {
        return <div className="p-4 text-center text-gray-500 dark:text-gray-50 h-18 ">No items found</div>;
    }

    return (
        <div className="h-full">
            <Virtuoso
                ref={virtuoso}
                style={{ height: '100%' }}
                data={getEarthquakes()}
                rangeChanged={setVisibleRange}
                itemContent={(index, item) => (
                    <div
                        key={index}
                        onClick={() => {
                            setSelected(item.id);
                        }}
                        className={cn(
                            'p-3 cursor-pointer border-b h-18',
                            selected === item.id ? 'bg-blue-100 dark:bg-sidebar-primary' : 'hover:bg-muted',
                        )}
                    >
                        <h4 className="text-md font-semibold text-ellipsis overflow-hidden whitespace-nowrap">
                            {item.properties.place}
                        </h4>
                        <p className="text-sm text-gray-500 dark:text-gray-50 text-ellipsis overflow-hidden whitespace-nowrap">
                            {format(item.properties.time, 'PPPpp')}
                        </p>
                    </div>
                )}
            />
        </div>
    );
}

export default EventList;
