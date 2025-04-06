import Button from '@/components/Button';
import useStore from '@/lib/store';
import { useParams } from 'react-router';

function Main() {
    const { id } = useParams();
    const counter = useStore((state) => state.counter);
    const incrementCounter = useStore((state) => state.increment);
    const decrementCounter = useStore((state) => state.decrement);
    const resetCounter = useStore((state) => state.reset);

    return (
        <div>
            Main page (id = {id ?? 'Not specified'})
            <div>
                <Button
                    disabled={counter === 0}
                    onClick={decrementCounter}
                >
                    -
                </Button>
                <Button onClick={incrementCounter}>+</Button>
                <Button onClick={resetCounter}>Reset</Button>
            </div>
        </div>
    );
}

export default Main;
