import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export type FavoriteLocation = {
  id: string;
  name: string;
  lat: number;
  lng: number;
  disaster: string;
  dangerLevel: 'low' | 'medium' | 'high';
  photo?: string | null;
  radius: number;
};

interface FavoritesState {
  favorites: FavoriteLocation[];
  addFavorite: (fav: FavoriteLocation) => void;
  removeFavorite: (id: string) => void;
  updateFavorite: (id: string, updatedFav: Partial<FavoriteLocation>) => void;
}

const useFavoritesStore = create<FavoritesState>()(
  persist(
    (set) => ({
      favorites: [], 

      addFavorite: (fav) =>
        set((state) => ({
          favorites: [...state.favorites, fav], 
        })),

      removeFavorite: (id) =>
        set((state) => ({
          favorites: state.favorites.filter((f) => f.id !== id), 
        })),

      updateFavorite: (id, updatedFav) =>
        set((state) => ({
          favorites: state.favorites.map((fav) =>
            fav.id === id ? { ...fav, ...updatedFav } : fav 
          ),
        })),
    }),
    {
      name: 'favorites-storage', 
      partialize: (state) => ({ favorites: state.favorites }),
    }
  )
);

export default useFavoritesStore;
