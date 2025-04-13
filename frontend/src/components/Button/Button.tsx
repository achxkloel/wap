import styles from './Button.module.scss';

interface ButtonProps {
    className?: string;
    onClick?: () => void;
    style?: React.CSSProperties;
    disabled?: boolean;
    children?: React.ReactNode;
    variant?: 'default' | 'light' | 'dark'| 'add';
    type?: "button" | "submit" | "reset";
}

function Button({ className, onClick, style, disabled, children, variant = 'default' }: ButtonProps) {
    const variantStyle = styles[`btn-${variant}`];

    return (
        <button
            className={`${styles.btn} ${variantStyle} ${className}`}
            onClick={onClick}
            style={style}
            disabled={disabled}
        >
            {children}
        </button>
    );
}

export default Button;
