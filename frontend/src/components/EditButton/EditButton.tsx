import React from 'react';
import { FaEdit } from 'react-icons/fa';
import styles from '@/components/Button/Button.module.scss';

type EditButtonProps = {
  onClick: () => void;
};

const EditButton: React.FC<EditButtonProps> = ({ onClick }) => {
  return (
    <button onClick={onClick} className={styles.iconButton}>
      <FaEdit />
    </button>
  );
};

export default EditButton;
