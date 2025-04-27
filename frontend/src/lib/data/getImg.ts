import defaultLocationImage from '@/assets/default_location.png';
import axios from 'axios';

const UNSPLASH_URL = 'https://api.unsplash.com/search/photos';
const DEFAULT_IMG_URL = defaultLocationImage;
const UNSPLASH_ACCESS_KEY = 'QomxOmr0uAe3rY0cL076U6MDaOCaBWfrd0DhQjmQCIo';

export interface UnsplashImageResult {
    urls: {
        regular: string;
    };
}

export interface UnsplashApiResponse {
    results: UnsplashImageResult[];
}

const getImg = async (location: string): Promise<string> => {
    try {
        const response = await axios.get<UnsplashApiResponse>(UNSPLASH_URL, {
            params: {
                query: location,
                client_id: UNSPLASH_ACCESS_KEY,
            },
        });

        const imgUrl = response.data?.results?.[0]?.urls?.regular ?? DEFAULT_IMG_URL;
        return imgUrl;
    } catch (error) {
        console.error('Error fetching image:', error);
        return DEFAULT_IMG_URL;
    }
};

export default getImg;
