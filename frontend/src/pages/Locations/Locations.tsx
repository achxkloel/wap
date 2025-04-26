import DeleteModal from '@/components/DeleteModal/DeleteModal';
import Map from '@/components/Map';
import { Button } from '@/components/ui/button';
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs';
import api from '@/lib/api';
import useAuthStore from '@/lib/store/auth';
import useLocationStore from '@/lib/store/location';
import { LayoutGrid, LayoutPanelLeftIcon, PlusIcon } from 'lucide-react';
import React, { useEffect, useState } from 'react';
import LocationCard from './LocationCard';
import LocationList from './LocationList';
import LocationModal from './LocationModal';
import { LocationFormValues } from './LocationModal/LocationModal';

const defaultValues: LocationFormValues = {
    name: '',
    description: '',
    photo: undefined,
    latitude: 51.477928,
    longitude: -0.001545,
    radius: 25,
};

function Locations() {
    const locations = useLocationStore((state) => state.locations);
    const setLocations = useLocationStore((state) => state.setLocations);
    const user = useAuthStore((state) => state.user);

    const [isModalOpen, setIsModalOpen] = useState(false);
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
    const [locationData, setLocationData] = useState<LocationFormValues>(defaultValues);
    const [selectedTab, setSelectedTab] = useState('card');
    const [selectedLocation, setSelectedLocation] = useState<any | null>(null);
    const [selectedListLocation, setSelectedListLocation] = useState<any | null>(null);

    useEffect(() => {
        fetchLocations();
    }, []);

    const fetchLocations = async () => {
        try {
            const res = await api.get('/natural_phenomenon_locations');
            const data = res.data;

            for (let i = 0; i < data.length; i++) {
                if (data[i].image_path) {
                    const image_path = `/${data[i].image_path}`;

                    const response = await api.get(image_path, {
                        responseType: 'blob',
                    });

                    const imageBlob = response.data;
                    const imageObjectURL = URL.createObjectURL(imageBlob);
                    data[i].image = imageObjectURL;
                }
            }

            setLocations(data);
        } catch (e) {
            console.error('Error fetching locations:', e);
        }
    };

    const createLocation = async (location: LocationFormValues) => {
        if (!user) {
            return;
        }

        const formData = new FormData();
        formData.append('user_id', user.id.toString());
        formData.append('name', location.name);
        formData.append('description', location.description);
        formData.append('latitude', location.latitude.toString());
        formData.append('longitude', location.longitude.toString());
        formData.append('radius', location.radius.toString());

        if (location.photo) {
            formData.append('image', location.photo);
        }

        try {
            await api.post('/natural_phenomenon_locations', formData, {
                headers: {
                    'Content-Type': 'multipart/form-data',
                },
            });

            fetchLocations();
        } catch (e) {
            console.error('Error creating location:', e);
        }
    };

    const editLocation = async (location: LocationFormValues) => {
        if (!selectedLocation) {
            return;
        }

        try {
            await api.put(`/natural_phenomenon_locations/${selectedLocation.id}`, {
                name: location.name,
                description: location.description,
                latitude: location.latitude,
                longitude: location.longitude,
                radius: location.radius,
            });
            fetchLocations();
        } catch (e) {
            console.error('Error editing location:', e);
        } finally {
            setSelectedLocation(null);
            setSelectedListLocation(null);
        }
    };

    const deleteLocation = async () => {
        if (!selectedLocation) {
            return;
        }

        try {
            await api.delete(`/natural_phenomenon_locations/${selectedLocation.id}`);
            fetchLocations();
        } catch (e) {
            console.error('Error deleting location:', e);
        } finally {
            setSelectedLocation(null);
            setSelectedListLocation(null);
            setIsDeleteModalOpen(false);
        }
    };

    const handleSaveClick = async (location: LocationFormValues) => {
        setIsModalOpen(false);

        if (selectedLocation) {
            await editLocation(location);
        } else {
            await createLocation(location);
        }
    };

    const handleCreateClick = () => {
        setLocationData(defaultValues);
        setSelectedLocation(null);
        setIsModalOpen(true);
    };

    const handleEditClick = (location: any) => {
        setSelectedLocation(location);
        setLocationData({
            name: location.name,
            description: location.description,
            photo: undefined,
            latitude: location.latitude,
            longitude: location.longitude,
            radius: location.radius,
        });
        setIsModalOpen(true);
    };

    const handleDeleteClick = (location: any) => {
        setSelectedLocation(location);
        setIsDeleteModalOpen(true);
    };

    const handleTabChange = (value: string) => {
        if (value === 'card') {
            setSelectedListLocation(null);
        } else {
            if (locations && locations.length > 0) {
                setSelectedListLocation(locations[0]);
            }
        }

        setSelectedTab(value);
    };

    return (
        <div className="h-full w-full flex flex-col">
            <div className="border-b p-2 flex justify-between items-center bg-sidebar">
                <Button
                    variant="outline"
                    onClick={handleCreateClick}
                >
                    New location <PlusIcon />
                </Button>
                <Tabs
                    defaultValue="card"
                    value={selectedTab}
                    onValueChange={handleTabChange}
                >
                    <TabsList>
                        <TabsTrigger value="card">
                            <LayoutGrid />
                        </TabsTrigger>
                        <TabsTrigger value="map">
                            <LayoutPanelLeftIcon />
                        </TabsTrigger>
                    </TabsList>
                </Tabs>
            </div>
            {selectedTab === 'card' ? (
                <React.Fragment>
                    {!locations || locations.length <= 0 ? (
                        <div className="h-full mx-auto flex flex-col items-center justify-center">
                            <h1 className="text-lg font-semibold text-gray-500 dark:text-gray-50">
                                No locations found
                            </h1>
                        </div>
                    ) : (
                        <div className="h-full overflow-y-scroll">
                            <div className="container mx-auto grid-cols-1 xl:grid-cols-2 grid gap-4 p-4">
                                {locations.map((location: any, index: number) => (
                                    <LocationCard
                                        key={index}
                                        location={location}
                                        onEdit={handleEditClick}
                                        onDelete={handleDeleteClick}
                                    />
                                ))}
                            </div>
                        </div>
                    )}
                </React.Fragment>
            ) : (
                <div className="h-full w-full flex">
                    <Map
                        location={
                            selectedListLocation
                                ? [selectedListLocation.latitude, selectedListLocation.longitude]
                                : undefined
                        }
                        locationRadius={selectedListLocation ? selectedListLocation.radius : undefined}
                        locationZoom={7}
                    />
                    <div className="flex flex-col w-[600px] gap-2 bg-sidebar">
                        <LocationList
                            selected={selectedListLocation}
                            onChange={(location) => {
                                if (selectedListLocation && selectedListLocation.id === location.id) {
                                    setSelectedListLocation(null);
                                    return;
                                }

                                setSelectedListLocation(location);
                            }}
                            onEdit={handleEditClick}
                            onDelete={handleDeleteClick}
                        />
                    </div>
                </div>
            )}

            <LocationModal
                open={isModalOpen}
                onOpenChange={setIsModalOpen}
                defaultValues={defaultValues}
                values={locationData}
                onSubmit={handleSaveClick}
                edit={!!selectedLocation}
            />
            <DeleteModal
                title={selectedLocation ? `Delete "${selectedLocation.name}"` : ''}
                description="Are you sure you want to delete this location?"
                open={isDeleteModalOpen}
                onOpenChange={setIsDeleteModalOpen}
                onDelete={deleteLocation}
            />
        </div>
    );
}

export default Locations;
