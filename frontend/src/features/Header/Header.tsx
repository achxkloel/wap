import useStore from '@/lib/store';
import { Link } from 'react-router';
import { AuthDialog } from '@/components/AuthDialog';

function Header() {
    const counter = useStore((state) => state.counter);

    return (
        <header className="w-full bg-white shadow-md">
            <div className="container mx-auto px-4 py-3 flex items-center justify-between">
                <nav className="flex gap-6 text-gray-700 font-medium">
                    <Link to="/" className="hover:text-blue-600 transition">
                        Main
                    </Link>
                    <Link to="/settings" className="hover:text-blue-600 transition">
                        Settings
                    </Link>
                </nav>

                <div className="flex items-center gap-4">
                    <div className="text-sm text-gray-600 bg-gray-100 px-3 py-1 rounded-full">
                        Counter: {counter}
                    </div>
                    <AuthDialog />
                </div>
            </div>
        </header>
    );
}

export default Header;
