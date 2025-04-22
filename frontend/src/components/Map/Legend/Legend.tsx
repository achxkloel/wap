import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import useMapStore from '@/lib/store/map';
import { differenceInMilliseconds, Duration, sub } from 'date-fns';
import L from 'leaflet';

const magnitudeColors = [
    {
        label: '1-2',
        color: '#ffffb2',
    },
    {
        label: '3',
        color: '#fed976',
    },
    {
        label: '4',
        color: '#feb24c',
    },
    {
        label: '5',
        color: '#fd8d3c',
    },
    {
        label: '6',
        color: '#fc4e2a',
    },
    {
        label: '7+',
        color: '#e31a1c',
    },
];

const significanceColors = [
    {
        label: '1-99',
        color: '#ffffb2',
    },
    {
        label: '100',
        color: '#fed976',
    },
    {
        label: '300',
        color: '#feb24c',
    },
    {
        label: '500',
        color: '#fd8d3c',
    },
    {
        label: '700',
        color: '#fc4e2a',
    },
    {
        label: '900+',
        color: '#e31a1c',
    },
];

const dateColors = [
    {
        label: '<1h',
        color: '#e31a1c',
    },
    {
        label: '1h',
        color: '#fd8d3c',
    },
    {
        label: '1d',
        color: '#fed976',
    },
    {
        label: '7d',
        color: '#ffffb2',
    },
    {
        label: '30d+',
        color: '#cccccc',
    },
];

const magnitudeSizes = [
    {
        label: '1-2',
        size: 20,
    },
    {
        label: '3',
        size: 22,
    },
    {
        label: '4',
        size: 24,
    },
    {
        label: '5',
        size: 26,
    },
    {
        label: '6',
        size: 28,
    },
    {
        label: '7+',
        size: 30,
    },
];

export const getMagnitudeColor = (magnitude: number) => {
    if (magnitude < 3) {
        return magnitudeColors[0];
    }
    if (magnitude < 4) {
        return magnitudeColors[1];
    }
    if (magnitude < 5) {
        return magnitudeColors[2];
    }
    if (magnitude < 6) {
        return magnitudeColors[3];
    }
    if (magnitude < 7) {
        return magnitudeColors[4];
    }

    return magnitudeColors[5];
};

export const getSignificanceColor = (significance: number) => {
    if (significance < 100) {
        return significanceColors[0];
    }
    if (significance < 300) {
        return significanceColors[1];
    }
    if (significance < 500) {
        return significanceColors[2];
    }
    if (significance < 700) {
        return significanceColors[3];
    }
    if (significance < 900) {
        return significanceColors[4];
    }

    return significanceColors[5];
};

export const getDateColor = (date: number) => {
    if (compareDate(date, { hours: 1 })) {
        return dateColors[0];
    }
    if (compareDate(date, { days: 1 })) {
        return dateColors[1];
    }
    if (compareDate(date, { days: 7 })) {
        return dateColors[2];
    }
    if (compareDate(date, { days: 30 })) {
        return dateColors[3];
    }
    return dateColors[4];
};

export const getSize = (magnitude: number) => {
    if (magnitude < 3) {
        return magnitudeSizes[0];
    }
    if (magnitude < 4) {
        return magnitudeSizes[1];
    }
    if (magnitude < 5) {
        return magnitudeSizes[2];
    }
    if (magnitude < 6) {
        return magnitudeSizes[3];
    }
    if (magnitude < 7) {
        return magnitudeSizes[4];
    }
    return magnitudeSizes[5];
};

const compareDate = (date: number, duration: Duration) => {
    const now = new Date();
    const targetDate: Date = sub(now, duration);
    const diff = differenceInMilliseconds(now, date);
    return diff < differenceInMilliseconds(now, targetDate);
};

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
            className="z-[1000] absolute right-5 bottom-5 bg-background p-2 rounded border-[1px] border-solid border-border cursor-default space-y-4"
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
                                className="rounded-full border-black border-solid border-[1px] bg-white"
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
