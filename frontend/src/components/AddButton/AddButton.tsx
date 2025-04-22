import Button from '@/components/Button/Button';

const AddButton: React.FC<{ onClick: () => void }> = ({ onClick }) => {
  return (
    <div className="w-full m-2">
      <Button onClick={onClick} variant="add">
        <span className="plus-icon">+</span> Add new location
      </Button>
    </div>
  );
};

export default AddButton;
