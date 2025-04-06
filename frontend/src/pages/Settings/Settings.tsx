import Button from '@/components/Button';
import useStore from '@/lib/store';

function Settings() {
    const theme = useStore((state) => state.theme);
    const toggleTheme = useStore((state) => state.toggleTheme);

    return (
        <div>
            Settings page
            <div>
                <Button
                    onClick={toggleTheme}
                    className="capitalize"
                    variant={theme}
                >
                    {theme}
                </Button>
            </div>
        </div>
    );
}

export default Settings;
