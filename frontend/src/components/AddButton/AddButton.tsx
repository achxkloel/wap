import Button from '@/components/Button/Button';

const AddButton: React.FC<{ onClick: () => void }> = ({ onClick }) => {
  return (
    <div>
      <Button onClick={onClick} variant="add">
        <span className="plus-icon">+</span> Add new location
      </Button>
    </div>
  );
};

export default AddButton;
