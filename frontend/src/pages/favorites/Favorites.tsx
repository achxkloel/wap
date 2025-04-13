import { useState, useEffect } from 'react';
import useFavoritesStore from '@/lib/favorites';
import LocationCard from '@/components/LocationCard/LocationCard';
import AddButton from '@/components/AddButton/AddButton';
import Modal from '@/components/ModalWindow/Modal';
import locationImage from '@/assets/location.png';
import styles from '@/components/LocationCard/LocationCard.module.scss';

export default function FavoritesPage() {
  const favorites = useFavoritesStore((s) => s.favorites);
  const addFavorite = useFavoritesStore((s) => s.addFavorite);
  const updateFavorite = useFavoritesStore((s) => s.updateFavorite);
  const removeFavorite = useFavoritesStore((s) => s.removeFavorite);

  const [isModalOpen, setIsModalOpen] = useState(false); 
  const [selectedLocation, setSelectedLocation] = useState<any | null>(null); 

  useEffect(() => {
    if (favorites.length === 0) {
      addFavorite({
        id: '1',
        name: 'Tokyo',
        lat: 35.6895,
        lng: 139.6917,
        disaster: 'Flood',
        dangerLevel: 'high',
        photo: locationImage,
        radius: 50,
      });
      addFavorite({
        id: '2',
        name: 'New York',
        lat: 40.7128,
        lng: -74.0060,
        disaster: 'Earthquake',
        dangerLevel: 'medium',
        photo: locationImage,
        radius: 30,
      });
      addFavorite({
        id: '3',
        name: 'Paris',
        lat: 48.8566,
        lng: 2.3522,
        disaster: 'Tornado',
        dangerLevel: 'low',
        photo: locationImage,
        radius: 40,
      });
    }
  }, [favorites.length, addFavorite]);

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
    <div className={styles.pageContainer}>
      <div className={styles.menu}>
        <p>Main Menu</p>
      </div>

      <div className={styles.content}>
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-2xl font-bold">Favorite Locations</h1>
          <AddButton onClick={() => setIsModalOpen(true)} />
        </div>

        {favorites.length === 0 ? (
          <p className="text-gray-500">You haven’t added any favorite locations yet.</p>
        ) : (
          <ul className="space-y-4">
            {favorites.map((fav) => (
              <LocationCard
                key={fav.id}
                locationName={fav.name}
                weather="Sunny"
                temperature="20°C"
                disaster={fav.disaster}
                riskLevel={`Risk: ${fav.dangerLevel}`}
                photo={fav.photo}
                onEdit={() => handleEditLocation(fav)} 
                onDelete={() => handleDeleteLocation(fav.id)} 
              />
            ))}
          </ul>
        )}
      </div>

      <Modal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)} 
        onSave={handleSaveLocation}
        existingLocation={selectedLocation}
      />
    </div>
  );
}
