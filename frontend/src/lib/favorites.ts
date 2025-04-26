import { create } from 'zustand';

type FavoriteLocation = {
    id: string;
    name: string;
    lat: number;
    lng: number;
    disaster: string;
    dangerLevel: string;
    photo?: string | null;
    radius: number;
};

interface FavoritesState {
    favorites: FavoriteLocation[];
    addFavorite: (fav: FavoriteLocation) => void;
    updateFavorite: (id: string, updatedFav: Partial<FavoriteLocation>) => void;
    removeFavorite: (id: string) => void;
}

const useFavoritesStore = create<FavoritesState>((set) => ({
    favorites: [],

    addFavorite: (fav) => {
        set((state) => ({
            favorites: [...state.favorites, fav],
        }));
        fetch('/location', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(fav),
        })
            .then((res) => res.json())
            .catch((err) => console.error('Error:', err));
    },

    updateFavorite: (id, updatedFav) => {
        set((state) => ({
            favorites: state.favorites.map((fav) => (fav.id === id ? { ...fav, ...updatedFav } : fav)),
        }));
        fetch(`/location/${id}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(updatedFav),
        })
            .then((res) => res.json())
            .catch((err) => console.error('Error:', err));
    },

    removeFavorite: (id) => {
        set((state) => ({
            favorites: state.favorites.filter((f) => f.id !== id),
        }));
        fetch(`/location/${id}`, {
            method: 'DELETE',
        })
            .then((res) => res.json())
            .catch((err) => console.error('Error:', err));
    },
}));

export default useFavoritesStore;
