import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import useMapStore from '@/lib/store/map';
import { differenceInMilliseconds, Duration, sub } from 'date-fns';
import L from 'leaflet';

export const magnitudeColors = [
    {
        cond: (magnitude: number) => magnitude < 3,
        label: '1-2',
        color: '#ffffb2',
    },
    {
        cond: (magnitude: number) => magnitude < 4,
        label: '3',
        color: '#fed976',
    },
    {
        cond: (magnitude: number) => magnitude < 5,
        label: '4',
        color: '#feb24c',
    },
    {
        cond: (magnitude: number) => magnitude < 6,
        label: '5',
        color: '#fd8d3c',
    },
    {
        cond: (magnitude: number) => magnitude < 7,
        label: '6',
        color: '#fc4e2a',
    },
    {
        cond: (magnitude: number) => magnitude >= 7,
        label: '7+',
        color: '#e31a1c',
    },
];

export const significanceColors = [
    {
        cond: (significanceLevel: number) => significanceLevel < 100,
        label: '1-99',
        color: '#ffffb2',
    },
    {
        cond: (significanceLevel: number) => significanceLevel < 300,
        label: '100',
        color: '#fed976',
    },
    {
        cond: (significanceLevel: number) => significanceLevel < 500,
        label: '300',
        color: '#feb24c',
    },
    {
        cond: (significanceLevel: number) => significanceLevel < 700,
        label: '500',
        color: '#fd8d3c',
    },
    {
        cond: (significanceLevel: number) => significanceLevel < 900,
        label: '700',
        color: '#fc4e2a',
    },
    {
        cond: (significanceLevel: number) => significanceLevel >= 900,
        label: '900+',
        color: '#e31a1c',
    },
];

const compareDate = (date: number, duration: Duration) => {
    const now = new Date();
    const targetDate: Date = sub(now, duration);
    const diff = differenceInMilliseconds(now, date);
    return diff < differenceInMilliseconds(now, targetDate);
};

export const dateColors = [
    {
        cond: (date: number) => compareDate(date, { hours: 1 }),
        label: '1h',
        color: '#e31a1c',
    },
    {
        cond: (date: number) => compareDate(date, { days: 1 }),
        label: '1d',
        color: '#fd8d3c',
    },
    {
        cond: (date: number) => compareDate(date, { days: 7 }),
        label: '7d',
        color: '#fed976',
    },
    {
        cond: (date: number) => compareDate(date, { days: 30 }),
        label: '30d',
        color: '#ffffb2',
    },
    {
        cond: (date: number) => compareDate(date, { days: 90 }),
        label: '90d+',
        color: '#cccccc',
    },
];

export const magnitudeSizes = [
    {
        cond: (magnitude: number) => magnitude < 3,
        label: '1-2',
        size: 20,
    },
    {
        cond: (magnitude: number) => magnitude < 4,
        label: '3',
        size: 22,
    },
    {
        cond: (magnitude: number) => magnitude < 5,
        label: '4',
        size: 24,
    },
    {
        cond: (magnitude: number) => magnitude < 6,
        label: '5',
        size: 26,
    },
    {
        cond: (magnitude: number) => magnitude < 7,
        label: '6',
        size: 28,
    },
    {
        cond: (magnitude: number) => magnitude >= 7,
        label: '7+',
        size: 30,
    },
];

function Legend() {
    const colorStrategy = useMapStore((state) => state.colorStrategy);
    const setColorStrategy = useMapStore((state) => state.setColorStrategy);

    const getColors = () => {
        switch (colorStrategy) {
            case 'magnitude':
                return magnitudeColors;
            case 'significance':
                return significanceColors;
            case 'date':
                return dateColors;
            default:
                return magnitudeColors;
        }
    };

    const stopPropagation = (el: HTMLDivElement | null) => {
        if (el === null) {
            return;
        }

        L.DomEvent.disableClickPropagation(el);
        L.DomEvent.disableScrollPropagation(el);
    };

    return (
        <div
            className="z-[1000] absolute right-5 bottom-5 bg-white p-2 rounded border-[1px] border-solid border-gray-400 cursor-default space-y-4"
            ref={stopPropagation}
        >
            <div className="space-y-2">
                <h4 className="text-md font-semibold">Color</h4>
                <Select
                    value={colorStrategy}
                    onValueChange={setColorStrategy}
                >
                    <SelectTrigger className="w-full">
                        <SelectValue />
                    </SelectTrigger>
                    <SelectContent className="z-[1000]">
                        <SelectGroup>
                            <SelectItem value="magnitude">By magnitude</SelectItem>
                            <SelectItem value="significance">By significance</SelectItem>
                            <SelectItem value="date">By date</SelectItem>
                        </SelectGroup>
                    </SelectContent>
                </Select>

                <div className="w-full flex justify-between gap-4">
                    {getColors().map((item) => (
                        <div
                            key={item.label}
                            className="flex flex-col items-center"
                        >
                            <div
                                className="w-5 h-5 border-black border-solid border-[1px]"
                                style={{ backgroundColor: item.color }}
                            ></div>
                            <span>{item.label}</span>
                        </div>
                    ))}
                </div>
            </div>
            <div className="space-y-2">
                <h4 className="text-md font-semibold">Size (by magnitude)</h4>
                <div className="flex items-end gap-4">
                    {magnitudeSizes.map((item) => (
                        <div
                            key={item.size}
                            className="flex flex-col items-center"
                        >
                            <div
                                className="rounded-full border-black border-solid border-[1px]"
                                style={{
                                    width: item.size,
                                    height: item.size,
                                }}
                            ></div>
                            <span>{item.label}</span>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}

export default Legend;
