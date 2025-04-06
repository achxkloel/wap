import Header from '@/features/Header';
import { Outlet } from 'react-router';
import styles from './Layout.module.css';

function Layout() {
    return (
        <div className={styles.container}>
            <Header />
            <div className={styles.content}>
                <Outlet />
            </div>
        </div>
    );
}

export default Layout;
