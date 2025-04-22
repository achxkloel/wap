import MapComponent from '@/components/Map';
import Page from '@/components/Page';
import Searchbar from '@/components/Searchbar';
import { Button } from '@/components/ui/button';
import getFilteredEarthquakes from '@/lib/data/earthquakes/getFiltered';
import useData from '@/lib/store/data';
import { ArrowDownAZIcon, ArrowUpAZIcon, FunnelIcon, XIcon } from 'lucide-react';
import React, { useEffect, useState } from 'react';
import EventList from './EventList';
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
    limit: 20000,
    orderBy: 'magnitude',
    orderDirection: 'desc',
};

function Map() {
    const [filters, setFilters] = useState<FilterFormValues>(defaultFilters);
    const [filterOpen, setFilterOpen] = useState(false);
    const [searchValue, setSearchValue] = useState<string>('');
    const [searchValueSubmitted, setSearchValueSubmitted] = useState<string>('');
    const setEarthquakes = useData((state) => state.setEarthquake);

    useEffect(() => {
        fetchData();
    }, [filters]);

    const fetchData = async () => {
        try {
            const data = await getFilteredEarthquakes(filters);
            setSearchValue('');
            setSearchValueSubmitted('');
            setEarthquakes(data);
        } catch (e) {
            console.error('Error fetching earthquakes:', e);
        }
    };

    const toggleFilter = () => {
        setFilterOpen((prev) => !prev);
    };

    const toggleOrderDirection = () => {
        setFilters((prev) => ({
            ...prev,
            orderDirection: prev.orderDirection === 'asc' ? 'desc' : 'asc',
        }));
    };

    const handleFilterSubmit = async (filters: FilterFormValues) => {
        toggleFilter();
        setFilters(filters);
    };

    return (
        <Page>
            <MapComponent />
            <div className="flex flex-col w-[400px] gap-2 bg-sidebar">
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
                    <React.Fragment>
                        <div className="w-full flex gap-2 p-4">
                            <Searchbar
                                value={searchValue}
                                onChange={setSearchValue}
                                iconPosition="left"
                                onSubmit={setSearchValueSubmitted}
                            />
                            <Button
                                variant="ghost"
                                size="icon"
                                onClick={toggleOrderDirection}
                                className="p-2"
                                disabled={filters.mode === 'realtime'}
                            >
                                {filters.orderDirection === 'asc' ? (
                                    <ArrowDownAZIcon className="size-4" />
                                ) : (
                                    <ArrowUpAZIcon className="size-4" />
                                )}
                            </Button>
                            <Button
                                variant="ghost"
                                size="icon"
                                onClick={toggleFilter}
                                className="p-2"
                            >
                                <FunnelIcon className="size-4" />
                            </Button>
                        </div>
                        <EventList search={searchValueSubmitted} />
                    </React.Fragment>
                )}
            </div>
        </Page>
    );
}

export default Map;
