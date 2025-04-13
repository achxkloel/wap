import React from 'react';
import Button from '@/components/Button/Button';
import { FaTrashAlt } from 'react-icons/fa';
import styles from '@/components/Button/Button.module.scss';

type DeleteButtonProps = {
  onClick: () => void;
};

const DeleteButton: React.FC<DeleteButtonProps> = ({ onClick }) => {
  return (
    <Button onClick={onClick} className={styles.iconButton}>
      <FaTrashAlt /> 
    </Button>
  );
};

export default DeleteButton;
