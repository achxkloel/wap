import useStore from '@/lib/store';
import { Link } from 'react-router';
import styles from './Header.module.css';

function Header() {
    const counter = useStore((state) => state.counter);

    return (
        <div className={styles.header}>
            <div className={styles.links}>
                <Link
                    to="/"
                    className={styles.link}
                >
                    Main
                </Link>
                <Link
                    to="/settings"
                    className={styles.link}
                >
                    Settings
                </Link>
            </div>
            <div className={styles.counter}>{counter}</div>
        </div>
    );
}

export default Header;
