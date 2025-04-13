import React from 'react';
import DeleteButton from '@/components/DeleteButton/DeleteButton'; 
import EditButton from '@/components/EditButton/EditButton'; 
import styles from '@/components/LocationCard/LocationCard.module.scss';
import locationImage from '@/assets/location.png';

interface LocationCardProps {
  locationName: string;
  weather: string;
  temperature: string;
  disaster: string;
  riskLevel: string;
  photo: string | null | undefined;
  onEdit: () => void;
  onDelete: () => void;
}

const LocationCard: React.FC<LocationCardProps> = ({
  locationName,
  weather,
  temperature,
  disaster,
  riskLevel,
  photo,
  onEdit,
  onDelete
}) => {
  return (
    <div className={styles.locationCard}>
      <div className={styles.locationCardHeader}>
        <img
          src={photo ? photo : locationImage}
          alt={locationName}
          className={styles.locationImage}
        />
      </div>

      <div className={styles.locationInfo}>
        <h4>{locationName}</h4>
        <div className={styles.weatherRow}>
          <p>{temperature},{weather}</p>
          <EditButton onClick={onEdit} />
        </div>
        <div className={styles.riskRow}>
          <p>{disaster} - {riskLevel}</p>
          <DeleteButton onClick={onDelete} />
        </div>
      </div>
    </div>
  );
};

export default LocationCard;
