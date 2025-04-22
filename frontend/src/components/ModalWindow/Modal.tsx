import React, { useState} from 'react';
import Button from '@/components/Button/Button';
import styles from '@/components/ModalWindow/ModalWindow.module.scss';
import Map from '@/components/Map';

interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (locationData: any) => void;
  existingLocation?: any;
}

const Modal: React.FC<ModalProps> = ({ isOpen, onClose, onSave, existingLocation }) => {
  const [locationName, setLocationName] = useState(existingLocation?.name || '');
  const [photo, setPhoto] = useState<File | null>(existingLocation?.photo || null);
  const [latitude, setLatitude] = useState(existingLocation?.lat || '');
  const [longitude, setLongitude] = useState(existingLocation?.lng || '');
  const [radius, setRadius] = useState(existingLocation?.radius || 50);
  const [showMap, setShowMap] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const locationData = { locationName, photo, latitude: parseFloat(latitude), longitude: parseFloat(longitude), radius };
    onSave(locationData); 
    onClose(); 
  };

  const handleMapClose = () => {
    setShowMap(false);
  };

  const handleCoordinateSelect = (lat: number, lng: number) => {
    setLatitude(lat.toString());
    setLongitude(lng.toString());
  };

  return (
    isOpen && (
      <div className={styles.modal}>
        <div className={styles.modalContent}>
          <h3>{existingLocation ? 'Edit Location' : 'Add New Location'}</h3>
          <form onSubmit={handleSubmit}>
            <div>
              <label>Location Name</label>
              <input
                type="text"
                value={locationName}
                onChange={(e) => setLocationName(e.target.value)}
                placeholder="Enter location name"
              />
            </div>
            <div>
              <label>Choose Photo</label>
              <input
                type="file"
                onChange={(e) => e.target.files && setPhoto(e.target.files[0])}
              />
            </div>

            <div>
              <label>Latitude</label>
              <input
                type="number"
                value={latitude}
                onChange={(e) => setLatitude(e.target.value)}
                placeholder="Enter Latitude"
              />
            </div>
            <div>
              <label>Longitude</label>
              <input
                type="number"
                value={longitude}
                onChange={(e) => setLongitude(e.target.value)}
                placeholder="Enter Longitude"
              />
            </div>

            <div>
              <Button onClick={() => setShowMap(true)}>Choose Coordinates on Map</Button>
            </div>

            {showMap && (
              <div className={styles.mapContainer}>
                <Map
                  location={[parseFloat(latitude), parseFloat(longitude)]}
                  locationZoom={13}
                  animate={true}
                  onCoordinateSelect={handleCoordinateSelect}
                />
                <Button onClick={handleMapClose}>Close Map</Button>
              </div>
            )}

            {latitude && longitude && (
              <div>
                <label>Coordinates</label>
                <input
                  type="text"
                  value={`Lat: ${latitude}, Lng: ${longitude}`}
                  readOnly
                />
              </div>
            )}

            <div>
              <label>Choose Radius</label>
              <input
                type="range"
                min="1"
                max="100"
                value={radius}
                onChange={(e) => setRadius(Number(e.target.value))}
              />
              <span>{radius} km</span>
            </div>

            <div className={styles.modalActions}>
              <Button type="submit">Save</Button>
              <Button onClick={onClose}>Cancel</Button>
            </div>
          </form>
        </div>
      </div>
    )
  );
};

export default Modal;
