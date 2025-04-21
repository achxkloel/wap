import MapComponent from '@/components/Map';
import Page from '@/components/Page';
import Searchbar from '@/components/Searchbar';
import { Button } from '@/components/ui/button';
import getFilteredEarthquakes from '@/lib/data/earthquakes/getFiltered';
import useData from '@/lib/store/data';
import { FunnelIcon, XIcon } from 'lucide-react';
import React, { useEffect, useState } from 'react';
import Filters from './Filters';
import { FilterFormValues } from './Filters/Filters';

const defaultFilters: FilterFormValues = {
    mode: 'realtime',
    realtimePeriod: 'week',
    realtimeMagnitude: 'all',
    startTime: undefined,
    endTime: undefined,
    catalog: undefined,
    contributor: undefined,
    productType: undefined,
    includeAllMagnitudes: false,
    includeAllOrigins: false,
    includeAllArrivals: false,
    minDepth: undefined,
    maxDepth: undefined,
    locationType: undefined,
    minLatitude: undefined,
    maxLatitude: undefined,
    minLongitude: undefined,
    maxLongitude: undefined,
    latitude: undefined,
    longitude: undefined,
    maxRadiusKm: undefined,
};

function Map() {
    const [filters, setFilters] = useState<FilterFormValues>(defaultFilters);
    const [filterOpen, setFilterOpen] = useState(false);
    const setEarthquakes = useData((state) => state.setEarthquake);

    useEffect(() => {
        fetchData();
    }, [filters]);

    const fetchData = async () => {
        try {
            const data = await getFilteredEarthquakes(filters);
            setEarthquakes(data);
        } catch (e) {
            console.error('Error fetching earthquakes:', e);
        }
    };

    const toggleFilter = () => {
        setFilterOpen((prev) => !prev);
    };

    const handleFilterSubmit = async (filters: FilterFormValues) => {
        toggleFilter();
        setFilters(filters);
    };

    return (
        <Page>
            <MapComponent />
            <div className="flex flex-col w-[400px] gap-2">
                {filterOpen ? (
                    <React.Fragment>
                        <div className="w-full flex justify-between p-4">
                            <h4 className="text-lg font-semibold">Filters</h4>
                            <Button
                                variant="ghost"
                                size="icon"
                                onClick={toggleFilter}
                                className="p-2"
                            >
                                <XIcon className="size-4" />
                            </Button>
                        </div>
                        <Filters
                            values={filters}
                            defaultValues={defaultFilters}
                            onSubmit={handleFilterSubmit}
                        />
                    </React.Fragment>
                ) : (
                    <div className="w-full flex gap-4 p-4">
                        <Searchbar
                            iconPosition="left"
                            onChange={(value) => {
                                console.log('Search value:', value);
                            }}
                            onSubmit={(value) => {
                                console.log('Search submitted:', value);
                            }}
                        />
                        <Button
                            variant="ghost"
                            size="icon"
                            onClick={toggleFilter}
                            className="p-2"
                        >
                            <FunnelIcon className="size-4" />
                        </Button>
                    </div>
                )}
            </div>
        </Page>
    );
}

export default Map;
