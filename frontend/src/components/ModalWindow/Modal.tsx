import React, { useState } from 'react';
import Button from '@/components/Button/Button';
import styles from '@/components/AddButton/ModalWindow.module.scss';

interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (locationData: any) => void;
  existingLocation?: any; 
}

const Modal: React.FC<ModalProps> = ({ isOpen, onClose, onSave, existingLocation }) => {
  const [locationName, setLocationName] = useState(existingLocation?.name || '');
  const [photo, setPhoto] = useState<File | null>(existingLocation?.photo || null);
  const [coordinates, setCoordinates] = useState(existingLocation?.coordinates || '');
  const [radius, setRadius] = useState(existingLocation?.radius || 50);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const locationData = { locationName, photo, coordinates, radius };
    onSave(locationData); 
    onClose(); 
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
              <label>Enter Coordinates</label>
              <input
                type="text"
                value={coordinates}
                onChange={(e) => setCoordinates(e.target.value)}
                placeholder="Enter coordinates"
              />
            </div>
            <div>
              <label>Choose Radius</label>
              <input
                type="range"
                min="1"
                max="100"
                value={radius}
                onChange={(e) => setRadius(Number(e.target.value))}
              />
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
