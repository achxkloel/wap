import { Button } from '@/components/ui/button';
import { Calendar } from '@/components/ui/calendar';
import { Checkbox } from '@/components/ui/checkbox';
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from '@/components/ui/command';
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Separator } from '@/components/ui/separator';
import { getCatalogEnum, getContributorEnum, getProductTypeEnum } from '@/lib/data/earthquakes/enums';
import { logger } from '@/lib/logger';
import useMapStore, { CircleCoordinates, RectangleCoordinates } from '@/lib/store/map';
import { cn, numberPreprocess } from '@/lib/utils';
import { zodResolver } from '@hookform/resolvers/zod';
import { format } from 'date-fns';
import L from 'leaflet';
import { CalendarIcon, Check, ChevronsUpDown } from 'lucide-react';
import React, { useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

const formSchema = z
    .object({
        mode: z.enum(['realtime', 'history']),
        realtimePeriod: z.enum(['hour', 'day', 'week', 'month']),
        realtimeMagnitude: z.enum(['all', '1.0', '2.5', '4.5', 'significant']),
        startTime: z.date().optional(),
        endTime: z.date().optional(),
        catalog: z.string().optional(),
        contributor: z.string().optional(),
        productType: z.string().optional(),
        includeAllMagnitudes: z.boolean().optional(),
        includeAllOrigins: z.boolean().optional(),
        includeAllArrivals: z.boolean().optional(),
        minDepth: z.preprocess(numberPreprocess, z.number().min(-100).max(100).nullish()),
        maxDepth: z.preprocess(numberPreprocess, z.number().min(-100).max(100).nullish()),
        minMagnitude: z.preprocess(numberPreprocess, z.number().positive().nullish()),
        maxMagnitude: z.preprocess(numberPreprocess, z.number().positive().nullish()),
        locationType: z.enum(['rectangle', 'circle']).optional(),
        minLatitude: z.preprocess(numberPreprocess, z.number().min(-90).max(90).nullish()),
        maxLatitude: z.preprocess(numberPreprocess, z.number().min(-90).max(90).nullish()),
        minLongitude: z.preprocess(numberPreprocess, z.number().min(-180).max(180).nullish()),
        maxLongitude: z.preprocess(numberPreprocess, z.number().min(-180).max(180).nullish()),
        latitude: z.preprocess(numberPreprocess, z.number().min(-90).max(90).nullish()),
        longitude: z.preprocess(numberPreprocess, z.number().min(-180).max(180).nullish()),
        maxRadiusKm: z.preprocess(numberPreprocess, z.number().positive().max(20001.6).nullish()),
        limit: z.preprocess(numberPreprocess, z.number().min(1).max(20000).nullish()),
        orderBy: z.enum(['time', 'magnitude']).optional(),
        orderDirection: z.enum(['asc', 'desc']).optional(),
    })
    .refine(
        (data) => {
            if (data.minDepth && data.maxDepth) {
                return data.minDepth <= data.maxDepth;
            }
            return true;
        },
        {
            message: 'Minimum depth must be less than maximum depth',
            path: ['minDepth'],
        },
    )
    .refine(
        (data) => {
            if (data.minMagnitude && data.maxMagnitude) {
                return data.minMagnitude <= data.maxMagnitude;
            }
            return true;
        },
        {
            message: 'Minimum magnitude must be less than maximum magnitude',
            path: ['minMagnitude'],
        },
    )
    .refine(
        (data) => {
            const params = [data.minLatitude, data.maxLatitude, data.minLongitude, data.maxLongitude];
            const defined = params.filter((param) => typeof param === 'number');
            return defined.length === 0 || defined.length === 4;
        },
        {
            message: 'All coordinates must be defined or empty',
            path: ['minLatitude'],
        },
    )
    .refine(
        (data) => {
            const params = [data.latitude, data.longitude, data.maxRadiusKm];
            const defined = params.filter((param) => typeof param === 'number');
            return defined.length === 0 || defined.length === 3;
        },
        {
            message: 'Coordinates and radius must be defined or empty',
            path: ['latitude'],
        },
    );

export type FilterFormValues = z.infer<typeof formSchema>;

interface FiltersProps {
    values?: FilterFormValues;
    defaultValues: FilterFormValues;
    onSubmit?: (values: FilterFormValues) => void;
}

function Filters(props: FiltersProps) {
    const [catalogs, setCatalogs] = React.useState<string[]>([]);
    const [contributors, setContributors] = React.useState<string[]>([]);
    const [productTypes, setProductTypes] = React.useState<string[]>([]);

    const startDraw = useMapStore((state) => state.startDraw);
    const stopDraw = useMapStore((state) => state.stopDraw);
    const drawCoordinates = useMapStore((state) => state.coordinates);
    const drawEnabled = useMapStore((state) => state.draw);
    const mapBounds = useMapStore((state) => state.bounds);

    const form = useForm<FilterFormValues>({
        resolver: zodResolver(formSchema),
        defaultValues: props.defaultValues,
        values: props.values,
    });

    const mode = form.watch('mode');
    const startTime = form.watch('startTime');
    const endTime = form.watch('endTime');
    const locationType = form.watch('locationType');
    const minLatitude = form.watch('minLatitude');
    const maxLatitude = form.watch('maxLatitude');
    const minLongitude = form.watch('minLongitude');
    const maxLongitude = form.watch('maxLongitude');
    const latitude = form.watch('latitude');
    const longitude = form.watch('longitude');
    const maxRadiusKm = form.watch('maxRadiusKm');

    useEffect(() => {
        fetchEnums();

        // return () => {
        //     form.reset();
        // };
    }, []);

    useEffect(() => {
        stopDraw();

        if (locationType === 'rectangle') {
            resetCircleLocation();
        } else if (locationType === 'circle') {
            resetRectangleLocation();
        }
    }, [locationType]);

    useEffect(() => {
        if (drawCoordinates) {
            if (locationType === 'rectangle') {
                const coords = drawCoordinates as RectangleCoordinates;
                setRectangleLocation(
                    coords.getSouthWest().lat,
                    coords.getNorthEast().lat,
                    coords.getSouthWest().lng,
                    coords.getNorthEast().lng,
                );
            } else if (locationType === 'circle') {
                const coords = drawCoordinates as CircleCoordinates;
                setCircleLocation(coords.center[0], coords.center[1], coords.radius);
            }
        }
    }, [drawCoordinates]);

    const fetchEnums = async () => {
        try {
            const catalogs = await getCatalogEnum();
            const contributors = await getContributorEnum();
            const productTypes = await getProductTypeEnum();
            setCatalogs(catalogs);
            setContributors(contributors);
            setProductTypes(productTypes);
        } catch (error) {
            logger.error('Error fetching enumeration values', error);
        }
    };

    const startTimeDisabled = (date: Date) => {
        const lowerBound = new Date('1900-01-01');
        const upperBound = endTime ? endTime : new Date();
        return date < lowerBound || date > upperBound;
    };

    const endTimeDisabled = (date: Date) => {
        const lowerBound = startTime ? startTime : new Date('1900-01-01');
        const upperBound = new Date();
        return date < lowerBound || date > upperBound;
    };

    const selectLocation = () => {
        if (!locationType) {
            return;
        }

        if (
            locationType === 'rectangle' &&
            typeof minLatitude === 'number' &&
            typeof maxLatitude === 'number' &&
            typeof minLongitude === 'number' &&
            typeof maxLongitude === 'number'
        ) {
            startDraw(locationType, {
                coordinates: L.latLngBounds([minLatitude, minLongitude], [maxLatitude, maxLongitude]),
            });
            return;
        }

        if (
            locationType === 'circle' &&
            typeof latitude === 'number' &&
            typeof longitude === 'number' &&
            typeof maxRadiusKm === 'number'
        ) {
            startDraw(locationType, {
                coordinates: {
                    center: [latitude, longitude],
                    radius: maxRadiusKm * 1000,
                },
            });
            return;
        }

        startDraw(locationType);
    };

    const setRectangleLocation = (
        minLatitude: number,
        maxLatitude: number,
        minLongitude: number,
        maxLongitude: number,
    ) => {
        form.setValue('minLatitude', minLatitude);
        form.setValue('maxLatitude', maxLatitude);
        form.setValue('minLongitude', minLongitude);
        form.setValue('maxLongitude', maxLongitude);
        updateRectangleLocation();
    };

    const setCircleLocation = (latitude: number, longitude: number, maxRadiusM: number) => {
        form.setValue('latitude', latitude);
        form.setValue('longitude', longitude);
        form.setValue('maxRadiusKm', maxRadiusM / 1000);
        updateCircleLocation();
    };

    const resetRectangleLocation = () => {
        form.setValue('minLatitude', undefined);
        form.setValue('maxLatitude', undefined);
        form.setValue('minLongitude', undefined);
        form.setValue('maxLongitude', undefined);
        updateRectangleLocation();
    };

    const resetCircleLocation = () => {
        form.setValue('latitude', undefined);
        form.setValue('longitude', undefined);
        form.setValue('maxRadiusKm', undefined);
    };

    const updateRectangleLocation = () => {
        form.trigger(['minLatitude', 'maxLatitude', 'minLongitude', 'maxLongitude']);
    };

    const updateCircleLocation = () => {
        form.trigger(['latitude', 'longitude', 'maxRadiusKm']);
    };

    const setLocationFromBounds = () => {
        if (!mapBounds) {
            return;
        }

        stopDraw();
        setRectangleLocation(
            mapBounds.getSouthWest().lat,
            mapBounds.getNorthEast().lat,
            mapBounds.getSouthWest().lng,
            mapBounds.getNorthEast().lng,
        );
    };

    const onSubmit = (values: FilterFormValues) => {
        stopDraw();

        if (props.onSubmit) {
            props.onSubmit(values);
        }
    };

    const onReset = () => {
        form.reset();
        stopDraw();
    };

    return (
        <Form {...form}>
            <form
                onSubmit={form.handleSubmit(onSubmit)}
                className="h-full flex flex-col overflow-y-auto"
                noValidate
                autoComplete="off"
            >
                <div className="h-full space-y-8 overflow-y-auto p-4">
                    <FormField
                        control={form.control}
                        name="mode"
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>Mode</FormLabel>
                                <Select
                                    onValueChange={field.onChange}
                                    defaultValue={field.value}
                                    value={field.value}
                                >
                                    <FormControl>
                                        <SelectTrigger>
                                            <SelectValue placeholder="Select a mode" />
                                        </SelectTrigger>
                                    </FormControl>
                                    <SelectContent>
                                        <SelectItem value="realtime">Real-time</SelectItem>
                                        <SelectItem value="history">History</SelectItem>
                                    </SelectContent>
                                </Select>
                                {/* <FormDescription></FormDescription> */}
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    {mode === 'realtime' ? (
                        <React.Fragment>
                            <div className="space-y-2">
                                <h4 className="text-md font-semibold leading-none">Parameters</h4>
                                <Separator />
                                <FormField
                                    control={form.control}
                                    name="realtimePeriod"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Period</FormLabel>
                                            <Select
                                                onValueChange={field.onChange}
                                                defaultValue={field.value}
                                                value={field.value}
                                            >
                                                <FormControl>
                                                    <SelectTrigger>
                                                        <SelectValue placeholder="Select a period" />
                                                    </SelectTrigger>
                                                </FormControl>
                                                <SelectContent>
                                                    <SelectItem value="hour">Last hour</SelectItem>
                                                    <SelectItem value="day">Last day</SelectItem>
                                                    <SelectItem value="week">Last week</SelectItem>
                                                    <SelectItem value="month">Last month</SelectItem>
                                                </SelectContent>
                                            </Select>
                                            {/* <FormDescription></FormDescription> */}
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <FormField
                                    control={form.control}
                                    name="realtimeMagnitude"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Magnitude</FormLabel>
                                            <Select
                                                onValueChange={field.onChange}
                                                defaultValue={field.value}
                                                value={field.value}
                                            >
                                                <FormControl>
                                                    <SelectTrigger>
                                                        <SelectValue placeholder="Select a magnitude" />
                                                    </SelectTrigger>
                                                </FormControl>
                                                <SelectContent>
                                                    <SelectItem value="all">All</SelectItem>
                                                    <SelectItem value="1.0">1.0+</SelectItem>
                                                    <SelectItem value="2.5">2.5+</SelectItem>
                                                    <SelectItem value="4.5">4.5+</SelectItem>
                                                    <SelectItem value="significant">Significant</SelectItem>
                                                </SelectContent>
                                            </Select>
                                            {/* <FormDescription></FormDescription> */}
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                            </div>
                        </React.Fragment>
                    ) : (
                        <React.Fragment>
                            <div className="space-y-2">
                                <h4 className="text-md font-semibold leading-none">Time</h4>
                                <Separator />
                                <FormField
                                    control={form.control}
                                    name="startTime"
                                    render={({ field }) => {
                                        return (
                                            <FormItem>
                                                <FormLabel>Start time</FormLabel>
                                                <Popover>
                                                    <PopoverTrigger asChild>
                                                        <FormControl>
                                                            <Button
                                                                variant={'outline'}
                                                                className={cn(
                                                                    'w-full pl-3 text-left font-normal',
                                                                    !field.value && 'text-muted-foreground',
                                                                )}
                                                            >
                                                                {field.value ? (
                                                                    format(field.value, 'PPP')
                                                                ) : (
                                                                    <span>Pick a start time</span>
                                                                )}
                                                                <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                                                            </Button>
                                                        </FormControl>
                                                    </PopoverTrigger>
                                                    <PopoverContent
                                                        className="w-auto p-0"
                                                        align="start"
                                                    >
                                                        <Calendar
                                                            mode="single"
                                                            selected={field.value}
                                                            onSelect={field.onChange}
                                                            disabled={startTimeDisabled}
                                                            initialFocus
                                                        />
                                                    </PopoverContent>
                                                </Popover>
                                                {/* <FormDescription></FormDescription> */}
                                                <FormMessage />
                                            </FormItem>
                                        );
                                    }}
                                />
                                <FormField
                                    control={form.control}
                                    name="endTime"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>End time</FormLabel>
                                            <Popover>
                                                <PopoverTrigger asChild>
                                                    <FormControl>
                                                        <Button
                                                            variant={'outline'}
                                                            className={cn(
                                                                'w-full pl-3 text-left font-normal',
                                                                !field.value && 'text-muted-foreground',
                                                            )}
                                                        >
                                                            {field.value ? (
                                                                format(field.value, 'PPP')
                                                            ) : (
                                                                <span>Pick an end time</span>
                                                            )}
                                                            <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                                                        </Button>
                                                    </FormControl>
                                                </PopoverTrigger>
                                                <PopoverContent
                                                    className="w-auto p-0"
                                                    align="start"
                                                >
                                                    <Calendar
                                                        mode="single"
                                                        selected={field.value}
                                                        onSelect={field.onChange}
                                                        disabled={endTimeDisabled}
                                                        initialFocus
                                                    />
                                                </PopoverContent>
                                            </Popover>
                                            {/* <FormDescription></FormDescription> */}
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                            </div>
                            <div className="space-y-2">
                                <h4 className="text-md font-semibold leading-none">Location</h4>
                                <Separator />
                                <FormField
                                    control={form.control}
                                    name="locationType"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Type</FormLabel>
                                            <Select
                                                onValueChange={field.onChange}
                                                defaultValue={field.value}
                                                value={field.value}
                                            >
                                                <FormControl>
                                                    <SelectTrigger>
                                                        <SelectValue placeholder="Select a type" />
                                                    </SelectTrigger>
                                                </FormControl>
                                                <SelectContent>
                                                    <SelectItem value="rectangle">Rectangle</SelectItem>
                                                    <SelectItem value="circle">Circle</SelectItem>
                                                </SelectContent>
                                            </Select>
                                            {/* <FormDescription></FormDescription> */}
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                {locationType === 'rectangle' ? (
                                    <React.Fragment>
                                        <FormItem>
                                            <FormLabel>Start coordinates</FormLabel>
                                            <FormField
                                                control={form.control}
                                                name="minLatitude"
                                                render={({ field }) => (
                                                    <FormItem>
                                                        <FormControl>
                                                            <Input
                                                                placeholder="Latitude"
                                                                {...field}
                                                                value={field.value ?? ''}
                                                            />
                                                        </FormControl>
                                                        <FormMessage />
                                                    </FormItem>
                                                )}
                                            />
                                            <FormField
                                                control={form.control}
                                                name="minLongitude"
                                                render={({ field }) => (
                                                    <FormItem>
                                                        <FormControl>
                                                            <Input
                                                                placeholder="Longitude"
                                                                {...field}
                                                                value={field.value ?? ''}
                                                            />
                                                        </FormControl>
                                                        <FormMessage />
                                                    </FormItem>
                                                )}
                                            />
                                        </FormItem>
                                        <FormItem>
                                            <FormLabel>End coordinates</FormLabel>
                                            <FormField
                                                control={form.control}
                                                name="maxLatitude"
                                                render={({ field }) => (
                                                    <FormItem>
                                                        <FormControl>
                                                            <Input
                                                                placeholder="Latitude"
                                                                {...field}
                                                                value={field.value ?? ''}
                                                            />
                                                        </FormControl>
                                                        <FormMessage />
                                                    </FormItem>
                                                )}
                                            />
                                            <FormField
                                                control={form.control}
                                                name="maxLongitude"
                                                render={({ field }) => (
                                                    <FormItem>
                                                        <FormControl>
                                                            <Input
                                                                placeholder="Longitude"
                                                                {...field}
                                                                value={field.value ?? ''}
                                                            />
                                                        </FormControl>
                                                        <FormMessage />
                                                    </FormItem>
                                                )}
                                            />
                                        </FormItem>
                                        <Button
                                            variant="outline"
                                            className="w-full"
                                            type="button"
                                            onClick={setLocationFromBounds}
                                        >
                                            Use map bounds
                                        </Button>
                                    </React.Fragment>
                                ) : (
                                    <React.Fragment>
                                        <FormItem>
                                            <FormLabel>Center coordinates</FormLabel>
                                            <FormField
                                                control={form.control}
                                                name="latitude"
                                                render={({ field }) => (
                                                    <FormItem>
                                                        <FormControl>
                                                            <Input
                                                                placeholder="Latitude"
                                                                {...field}
                                                                value={field.value ?? ''}
                                                            />
                                                        </FormControl>
                                                        <FormMessage />
                                                    </FormItem>
                                                )}
                                            />
                                            <FormField
                                                control={form.control}
                                                name="longitude"
                                                render={({ field }) => (
                                                    <FormItem>
                                                        <FormControl>
                                                            <Input
                                                                placeholder="Longitude"
                                                                {...field}
                                                                value={field.value ?? ''}
                                                            />
                                                        </FormControl>
                                                        <FormMessage />
                                                    </FormItem>
                                                )}
                                            />
                                        </FormItem>
                                        <FormField
                                            control={form.control}
                                            name="maxRadiusKm"
                                            render={({ field }) => (
                                                <FormItem>
                                                    <FormLabel>Radius [km]</FormLabel>
                                                    <FormControl>
                                                        <Input
                                                            placeholder="30"
                                                            {...field}
                                                            value={field.value ?? ''}
                                                        />
                                                    </FormControl>
                                                    <FormMessage />
                                                </FormItem>
                                            )}
                                        />
                                    </React.Fragment>
                                )}
                                {locationType && (
                                    <Button
                                        variant="outline"
                                        className="w-full"
                                        type="button"
                                        onClick={selectLocation}
                                        disabled={drawEnabled}
                                    >
                                        Select on map
                                    </Button>
                                )}
                            </div>
                            <div className="space-y-2">
                                <h4 className="text-md font-semibold leading-none">Other</h4>
                                <Separator />
                                <FormField
                                    control={form.control}
                                    name="catalog"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Catalog</FormLabel>
                                            <Popover>
                                                <PopoverTrigger asChild>
                                                    <FormControl>
                                                        <Button
                                                            variant="outline"
                                                            role="combobox"
                                                            className={cn(
                                                                'w-full justify-between',
                                                                !field.value && 'text-muted-foreground',
                                                            )}
                                                        >
                                                            {field.value || 'Select a catalog'}
                                                            <ChevronsUpDown className="opacity-50" />
                                                        </Button>
                                                    </FormControl>
                                                </PopoverTrigger>
                                                <PopoverContent
                                                    className="w-[200px] p-0"
                                                    align="start"
                                                >
                                                    <Command>
                                                        <CommandInput
                                                            placeholder="Search catalog"
                                                            className="h-9"
                                                        />
                                                        <CommandList>
                                                            <CommandEmpty>No catalogs found.</CommandEmpty>
                                                            <CommandGroup>
                                                                {catalogs.map((catalog) => (
                                                                    <CommandItem
                                                                        value={catalog}
                                                                        key={catalog}
                                                                        onSelect={() => {
                                                                            form.setValue('catalog', catalog);
                                                                        }}
                                                                    >
                                                                        {catalog}
                                                                        <Check
                                                                            className={cn(
                                                                                'ml-auto',
                                                                                catalog === field.value
                                                                                    ? 'opacity-100'
                                                                                    : 'opacity-0',
                                                                            )}
                                                                        />
                                                                    </CommandItem>
                                                                ))}
                                                            </CommandGroup>
                                                        </CommandList>
                                                    </Command>
                                                </PopoverContent>
                                            </Popover>
                                            {/* <FormDescription></FormDescription> */}
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <FormField
                                    control={form.control}
                                    name="contributor"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Contributor</FormLabel>
                                            <Popover>
                                                <PopoverTrigger asChild>
                                                    <FormControl>
                                                        <Button
                                                            variant="outline"
                                                            role="combobox"
                                                            className={cn(
                                                                'w-full justify-between',
                                                                !field.value && 'text-muted-foreground',
                                                            )}
                                                        >
                                                            {field.value || 'Select a contributor'}
                                                            <ChevronsUpDown className="opacity-50" />
                                                        </Button>
                                                    </FormControl>
                                                </PopoverTrigger>
                                                <PopoverContent
                                                    className="w-[200px] p-0"
                                                    align="start"
                                                >
                                                    <Command>
                                                        <CommandInput
                                                            placeholder="Search contributor"
                                                            className="h-9"
                                                        />
                                                        <CommandList>
                                                            <CommandEmpty>No contributors found.</CommandEmpty>
                                                            <CommandGroup>
                                                                {contributors.map((contributor) => (
                                                                    <CommandItem
                                                                        value={contributor}
                                                                        key={contributor}
                                                                        onSelect={() => {
                                                                            form.setValue('contributor', contributor);
                                                                        }}
                                                                    >
                                                                        {contributor}
                                                                        <Check
                                                                            className={cn(
                                                                                'ml-auto',
                                                                                contributor === field.value
                                                                                    ? 'opacity-100'
                                                                                    : 'opacity-0',
                                                                            )}
                                                                        />
                                                                    </CommandItem>
                                                                ))}
                                                            </CommandGroup>
                                                        </CommandList>
                                                    </Command>
                                                </PopoverContent>
                                            </Popover>
                                            {/* <FormDescription></FormDescription> */}
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <FormField
                                    control={form.control}
                                    name="productType"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Product type</FormLabel>
                                            <Popover>
                                                <PopoverTrigger asChild>
                                                    <FormControl>
                                                        <Button
                                                            variant="outline"
                                                            role="combobox"
                                                            className={cn(
                                                                'w-full justify-between',
                                                                !field.value && 'text-muted-foreground',
                                                            )}
                                                        >
                                                            {field.value || 'Select a product type'}
                                                            <ChevronsUpDown className="opacity-50" />
                                                        </Button>
                                                    </FormControl>
                                                </PopoverTrigger>
                                                <PopoverContent
                                                    className="w-[200px] p-0"
                                                    align="start"
                                                >
                                                    <Command>
                                                        <CommandInput
                                                            placeholder="Search product type"
                                                            className="h-9"
                                                        />
                                                        <CommandList>
                                                            <CommandEmpty>No product types found.</CommandEmpty>
                                                            <CommandGroup>
                                                                {productTypes.map((productType) => (
                                                                    <CommandItem
                                                                        value={productType}
                                                                        key={productType}
                                                                        onSelect={() => {
                                                                            form.setValue('productType', productType);
                                                                        }}
                                                                    >
                                                                        {productType}
                                                                        <Check
                                                                            className={cn(
                                                                                'ml-auto',
                                                                                productType === field.value
                                                                                    ? 'opacity-100'
                                                                                    : 'opacity-0',
                                                                            )}
                                                                        />
                                                                    </CommandItem>
                                                                ))}
                                                            </CommandGroup>
                                                        </CommandList>
                                                    </Command>
                                                </PopoverContent>
                                            </Popover>
                                            {/* <FormDescription></FormDescription> */}
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <FormItem>
                                    <FormLabel>Depth</FormLabel>
                                    <FormField
                                        control={form.control}
                                        name="minDepth"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormControl>
                                                    <Input
                                                        placeholder="0"
                                                        {...field}
                                                        value={field.value ?? ''}
                                                    />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                    <FormField
                                        control={form.control}
                                        name="maxDepth"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormControl>
                                                    <Input
                                                        placeholder="100"
                                                        {...field}
                                                        value={field.value ?? ''}
                                                    />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                </FormItem>
                                <FormItem>
                                    <FormLabel>Magnitude</FormLabel>
                                    <FormField
                                        control={form.control}
                                        name="minMagnitude"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormControl>
                                                    <Input
                                                        placeholder="0"
                                                        {...field}
                                                        value={field.value ?? ''}
                                                    />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                    <FormField
                                        control={form.control}
                                        name="maxMagnitude"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormControl>
                                                    <Input
                                                        placeholder="7"
                                                        {...field}
                                                        value={field.value ?? ''}
                                                    />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                </FormItem>
                                <FormField
                                    control={form.control}
                                    name="limit"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Results limit</FormLabel>
                                            <FormControl>
                                                <Input
                                                    placeholder="100"
                                                    {...field}
                                                    value={field.value ?? ''}
                                                />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <FormField
                                    control={form.control}
                                    name="orderBy"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Order by</FormLabel>
                                            <Select
                                                onValueChange={field.onChange}
                                                defaultValue={field.value}
                                                value={field.value}
                                            >
                                                <FormControl>
                                                    <SelectTrigger>
                                                        <SelectValue placeholder="Select order field" />
                                                    </SelectTrigger>
                                                </FormControl>
                                                <SelectContent>
                                                    <SelectItem value="magnitude">Magnitude</SelectItem>
                                                    <SelectItem value="time">Date</SelectItem>
                                                </SelectContent>
                                            </Select>
                                            {/* <FormDescription></FormDescription> */}
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <FormField
                                    control={form.control}
                                    name="includeAllMagnitudes"
                                    render={({ field }) => (
                                        <FormItem className="flex flex-row items-start space-x-3 space-y-0 rounded-md border p-4 shadow">
                                            <FormControl>
                                                <Checkbox
                                                    className="m-[2px]"
                                                    checked={field.value}
                                                    onCheckedChange={field.onChange}
                                                />
                                            </FormControl>
                                            <div className="space-y-1 leading-none">
                                                <FormLabel>Include all magnitudes</FormLabel>
                                                <FormDescription>
                                                    Specify if all magnitudes should be included.
                                                </FormDescription>
                                            </div>
                                        </FormItem>
                                    )}
                                />
                                <FormField
                                    control={form.control}
                                    name="includeAllOrigins"
                                    render={({ field }) => (
                                        <FormItem className="flex flex-row items-start space-x-3 space-y-0 rounded-md border p-4 shadow">
                                            <FormControl>
                                                <Checkbox
                                                    className="m-[2px]"
                                                    checked={field.value}
                                                    onCheckedChange={field.onChange}
                                                />
                                            </FormControl>
                                            <div className="space-y-1 leading-none">
                                                <FormLabel>Include all origins</FormLabel>
                                                <FormDescription>
                                                    Specify if all origins should be included.
                                                </FormDescription>
                                            </div>
                                        </FormItem>
                                    )}
                                />
                                <FormField
                                    control={form.control}
                                    name="includeAllArrivals"
                                    render={({ field }) => (
                                        <FormItem className="flex flex-row items-start space-x-3 space-y-0 rounded-md border p-4 shadow">
                                            <FormControl>
                                                <Checkbox
                                                    className="m-[2px]"
                                                    checked={field.value}
                                                    onCheckedChange={field.onChange}
                                                />
                                            </FormControl>
                                            <div className="space-y-1 leading-none">
                                                <FormLabel>Include all arrivals</FormLabel>
                                                <FormDescription>
                                                    Specify if phase arrivals should be included.
                                                </FormDescription>
                                            </div>
                                        </FormItem>
                                    )}
                                />
                            </div>
                        </React.Fragment>
                    )}
                </div>
                <div className="p-4 flex flex-col gap-2">
                    <Button
                        variant="outline"
                        type="button"
                        onClick={onReset}
                    >
                        Reset filters
                    </Button>
                    <Button type="submit">Apply filters</Button>
                </div>
            </form>
        </Form>
    );
}

export default Filters;
