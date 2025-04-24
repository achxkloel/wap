import locationImage from '@/assets/location.png';
import AddButton from '@/components/AddButton/AddButton';
import LocationCard from '@/components/LocationCard/LocationCard';
import getEarthquakes from '@/lib/data/getEarthquakes';
import useFavoritesStore from '@/lib/favorites';
import { useEffect, useState } from 'react';
import LocationModal from './LocationModal';

export default function FavoritesPage() {
    const favorites = useFavoritesStore((s) => s.favorites);
    const addFavorite = useFavoritesStore((s) => s.addFavorite);
    const updateFavorite = useFavoritesStore((s) => s.updateFavorite);
    const removeFavorite = useFavoritesStore((s) => s.removeFavorite);

    const [isModalOpen, setIsModalOpen] = useState(false);
    const [selectedLocation, setSelectedLocation] = useState<any | null>(null);
    const [earthquakeData, setEarthquakeData] = useState<any[]>([]);

    useEffect(() => {
        const fetchEarthquakesForLocations = async () => {
            for (const location of favorites) {
                const params = {
                    latitude: location.lat,
                    longitude: location.lng,
                    maxradius: location.radius,
                    limit: 1,
                };

                try {
                    const data = await getEarthquakes(params);

                    if (data.features && data.features.length > 0) {
                        const maxMagEarthquake = data.features.reduce((max: any, current: any) =>
                            current.properties.mag > max.properties.mag ? current : max,
                        );
                        setEarthquakeData((prevData) => [
                            ...prevData,
                            { locationId: location.id, earthquake: maxMagEarthquake },
                        ]);
                    } else {
                        setEarthquakeData((prevData) => [...prevData, { locationId: location.id, earthquake: null }]);
                    }
                } catch (error) {
                    console.error('Error fetching earthquakes:', error);
                }
            }
        };

        fetchEarthquakesForLocations();
    }, [favorites]);

    const handleSaveLocation = (locationData: any) => {
        if (selectedLocation) {
            updateFavorite(selectedLocation.id, locationData);
        } else {
            addFavorite({
                id: Date.now().toString(),
                name: locationData.locationName,
                lat: 0,
                lng: 0,
                disaster: 'Flood',
                dangerLevel: 'low',
                photo: locationData.photo || locationImage,
                radius: locationData.radius,
            });
        }
        setIsModalOpen(false);
    };

    const handleEditLocation = (fav: any) => {
        setSelectedLocation(fav);
        setIsModalOpen(true);
    };

    const handleDeleteLocation = (id: string) => {
        removeFavorite(id);
    };

    return (
        <div className="h-full w-full flex flex-col">
            <div className="flex justify-center items-center mb-4 w-full sticky top-0 bg-white z-10 shadow-md py-4">
                <h1 className="text-2xl font-bold text-center mt-2">Favorite Locations</h1>
            </div>

            <div className="flex justify-center mb-4 w-full sticky top-0 bg-white z-10 shadow-md py-4">
                <AddButton onClick={() => setIsModalOpen(true)} />
            </div>

            <div className="flex-1 overflow-y-auto p-4 w-full">
                {favorites.length === 0 ? (
                    <p className="text-gray-500 text-center">You haven’t added any favorite locations yet.</p>
                ) : (
                    <ul className="w-full space-y-4">
                        {favorites.map((fav) => {
                            const earthquake = earthquakeData.find((data) => data.locationId === fav.id);
                            return (
                                <LocationCard
                                    key={fav.id}
                                    locationName={fav.name}
                                    weather="Sunny"
                                    temperature="20°C"
                                    disaster={fav.disaster}
                                    riskLevel={`Risk: ${fav.dangerLevel}`}
                                    photo={fav.photo}
                                    earthquake={earthquake ? earthquake.earthquake : null}
                                    onEdit={() => handleEditLocation(fav)}
                                    onDelete={() => handleDeleteLocation(fav.id)}
                                    lat={fav.lat}
                                    lng={fav.lng}
                                    radius={fav.radius}
                                />
                            );
                        })}
                    </ul>
                )}
            </div>

            <LocationModal
                open={isModalOpen}
                onOpenChange={setIsModalOpen}
                defaultValues={{
                    name: '',
                    photo: undefined,
                    latitude: 51.477928,
                    longitude: -0.001545,
                    radius: 25,
                }}
                onSubmit={(data) => {
                    console.log('Location data:', data);
                    setIsModalOpen(false);
                }}
            />

            {/* <Modal
                isOpen={isModalOpen}
                onClose={() => setIsModalOpen(false)}
                onSave={handleSaveLocation}
                existingLocation={selectedLocation}
            /> */}
        </div>
    );
}
