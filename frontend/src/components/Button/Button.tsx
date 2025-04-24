import styles from './Button.module.scss';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

interface ButtonProps {
    className?: string;
    onClick?: () => void;
    style?: React.CSSProperties;
    disabled?: boolean;
    children?: React.ReactNode;
    variant?: 'default' | 'light' | 'dark' | 'add';
    type?: 'button' | 'submit' | 'reset';
    icon?: any;
}

function Button({ className, onClick, style, disabled, children, variant = 'default', icon }: ButtonProps) {
    const variantStyle = styles[`btn-${variant}`];

    return (
        <button
            className={`${styles.btn} ${variantStyle} ${className} ${icon ? styles.iconButton : ''}`}
            onClick={onClick}
            style={style}
            disabled={disabled}
        >
            {icon && <FontAwesomeIcon icon={icon} />}
            {children}
        </button>
    );
}

export default Button;
