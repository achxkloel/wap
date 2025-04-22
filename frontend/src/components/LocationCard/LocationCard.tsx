import React, { useEffect, useState } from 'react';
import { faTrashAlt, faPenToSquare } from '@fortawesome/free-solid-svg-icons';
import Button from '@/components/Button/Button'; 
import styles from '@/components/LocationCard/LocationCard.module.scss';
import locationImage from '@/assets/location.png';
import { EarthquakeData, EarthquakeProperties } from '@/lib/data/getEarthquakes';
import getEarthquakes from '@/lib/data/getEarthquakes';  

interface LocationCardProps {
  locationName: string;
  weather: string;
  temperature: string;
  disaster: string;
  riskLevel: string;
  photo: string | null | undefined;
  earthquake : any;
  onEdit: () => void;
  onDelete: () => void;
  lat: number;
  lng: number;
  radius: number;
}

const LocationCard: React.FC<LocationCardProps> = ({
  locationName,
  weather,
  temperature,
  disaster,
  riskLevel,
  photo,
  onEdit,
  onDelete,
  lat,
  lng,
  radius
}) => {
  const [earthquakeData, setEarthquakeData] = useState<EarthquakeProperties | null>(null);

  useEffect(() => {
    const fetchEarthquakes = async () => {
      const params = {
        latitude: lat,
        longitude: lng,
        maxradiuskm: radius,
        limit: 1,
        orderby: 'mag',
      };

      try {
        const data = await getEarthquakes(params);
        if (data.features && data.features.length > 0) {
          const maxMagEarthquake = data.features.reduce((max: EarthquakeData['features'][0], current: EarthquakeData['features'][0]) => 
            current.properties.mag > max.properties.mag ? current : max
          );
          setEarthquakeData(maxMagEarthquake);
        } else {
          setEarthquakeData(null);
        }
      } catch (error) {
        console.error('Error fetching earthquakes:', error);
      }
    };

    fetchEarthquakes();
  }, [lat, lng, radius]);

  return (
    <div className="m-2">
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
            <p>{temperature}, {weather}</p>
            <Button onClick={onEdit} icon={faPenToSquare} />
          </div>
          <div className={styles.riskRow}>
            <p>{disaster} - {riskLevel}</p>
            <Button onClick={onDelete} icon={faTrashAlt} />
          </div>

          {earthquakeData ? (
            <div className={styles.earthquakeInfo}>
              <h5>Earthquake Detected:</h5>
              <p>Location: {earthquakeData.place}</p>
              <p>Magnitude: {earthquakeData.mag}</p>
              <p>Time: {new Date(earthquakeData.time).toLocaleString()}</p>
              <a href={earthquakeData.url} target="_blank" rel="noopener noreferrer">
                More details
              </a>
            </div>
          ) : (
            <p>No recent earthquakes in this area. All is calm.</p>
          )}
        </div>
      </div>
    </div>
  );
};

export default LocationCard;
