import Map from '@/components/Map';
import { Button } from '@/components/ui/button';
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Slider } from '@/components/ui/slider';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Textarea } from '@/components/ui/textarea';
import useMapStore, { CircleCoordinates } from '@/lib/store/map';
import { getCurrentLocation, numberPreprocess } from '@/lib/utils';
import { zodResolver } from '@hookform/resolvers/zod';
import { Loader2Icon } from 'lucide-react';
import { useEffect, useState } from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

const formSchema = z.object({
    name: z.string().min(1, 'Name is required'),
    description: z.string(),
    photo: z
        .instanceof(File)
        .refine((file) => file.size < 5000000, {
            message: 'Location photo must be less than 5MB.',
        })
        .nullish(),
    latitude: z.preprocess(numberPreprocess, z.number().min(-90).max(90)),
    longitude: z.preprocess(numberPreprocess, z.number().min(-180).max(180)),
    radius: z.number().min(1).max(100),
});

export type LocationFormValues = z.infer<typeof formSchema>;

interface LocationModalProps {
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
    values?: LocationFormValues;
    defaultValues: LocationFormValues;
    onSubmit?: (location: LocationFormValues) => void;
    edit?: boolean;
}

function LocationModal(props: LocationModalProps) {
    const [locationLoading, setLocationLoading] = useState(false);
    const [selectedTab, setSelectedTab] = useState<string>('information');
    const drawEnabled = useMapStore((state) => state.draw);
    const drawCoordinates = useMapStore((state) => state.coordinates);
    const startDraw = useMapStore((state) => state.startDraw);

    const form = useForm<LocationFormValues>({
        resolver: zodResolver(formSchema),
        defaultValues: props.defaultValues,
        values: props.values,
    });

    const latitude = form.watch('latitude');
    const longitude = form.watch('longitude');
    const radius = form.watch('radius');

    useEffect(() => {
        if (!props.open) {
            form.reset();
            setSelectedTab('information');
        }
    }, [props.open]);

    useEffect(() => {
        if (!drawEnabled) {
            setSelectedTab('information');
        }
    }, [drawEnabled]);

    useEffect(() => {
        if (props.open && drawCoordinates) {
            const coords = drawCoordinates as CircleCoordinates;
            form.setValue('latitude', coords.center[0]);
            form.setValue('longitude', coords.center[1]);
            form.setValue('radius', (Math.round(coords.radius / 1000) as number) || 1);
            form.trigger(['latitude', 'longitude', 'radius']);
        }
    }, [drawCoordinates]);

    const onSubmit = async (data: LocationFormValues) => {
        if (props.onSubmit) {
            props.onSubmit(data);
        }
    };

    const handleTabChange = (value: string) => {
        setSelectedTab(value);

        if (value === 'map') {
            selectCoordinates();
        }
    };

    const selectCoordinates = () => {
        if (typeof latitude !== 'number' || typeof longitude !== 'number') {
            return;
        }

        setTimeout(() => {
            startDraw('circle', {
                coordinates: {
                    center: [latitude, longitude],
                    radius: radius * 1000,
                },
                maxRadius: 100 * 1000,
            });
        }, 1);
    };

    const useCurrentLocation = () => {
        setLocationLoading(true);

        getCurrentLocation()
            .then((position) => {
                const { latitude, longitude } = position.coords;
                form.setValue('latitude', latitude);
                form.setValue('longitude', longitude);
                form.trigger(['latitude', 'longitude']);
            })
            .finally(() => {
                setLocationLoading(false);
            });
    };

    return (
        <Dialog
            open={props.open}
            onOpenChange={props.onOpenChange}
        >
            <DialogContent className="w-fit max-w-fit">
                <DialogHeader>
                    <DialogTitle>New location</DialogTitle>
                </DialogHeader>

                <Form {...form}>
                    <form
                        onSubmit={form.handleSubmit(onSubmit)}
                        noValidate
                        className="space-y-4"
                        autoComplete="off"
                    >
                        <Tabs
                            defaultValue="information"
                            value={selectedTab}
                            onValueChange={handleTabChange}
                            className="w-full"
                        >
                            <TabsList className="w-full grid grid-cols-2">
                                <TabsTrigger value="information">Information</TabsTrigger>
                                <TabsTrigger value="map">Map</TabsTrigger>
                            </TabsList>
                            <TabsContent value="information">
                                <div className="w-[400px] space-y-4">
                                    <FormField
                                        control={form.control}
                                        name="name"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormLabel>Name</FormLabel>
                                                <FormControl>
                                                    <Input {...field} />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                    <FormField
                                        control={form.control}
                                        name="description"
                                        render={({ field }) => (
                                            <FormItem>
                                                <FormLabel>Description</FormLabel>
                                                <FormControl>
                                                    <Textarea
                                                        className="resize-none"
                                                        {...field}
                                                    />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                    {!props.edit && (
                                        <FormField
                                            control={form.control}
                                            name="photo"
                                            render={({ field: { value, onChange, ...fieldProps } }) => (
                                                <FormItem>
                                                    <FormLabel>Photo</FormLabel>
                                                    <FormControl>
                                                        <Input
                                                            {...fieldProps}
                                                            placeholder="Choose a file"
                                                            type="file"
                                                            accept="image/*"
                                                            onChange={(event) =>
                                                                onChange(event.target.files && event.target.files[0])
                                                            }
                                                        />
                                                    </FormControl>
                                                    <FormMessage />
                                                </FormItem>
                                            )}
                                        />
                                    )}
                                    <FormItem>
                                        <FormLabel>Cooridnates</FormLabel>
                                        <FormField
                                            control={form.control}
                                            name="latitude"
                                            disabled={locationLoading}
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
                                            disabled={locationLoading}
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
                                        onClick={() => {
                                            setSelectedTab('map');
                                            selectCoordinates();
                                        }}
                                    >
                                        Select on map
                                    </Button>
                                    <Button
                                        variant="outline"
                                        className="w-full"
                                        type="button"
                                        onClick={useCurrentLocation}
                                        disabled={locationLoading}
                                    >
                                        {locationLoading && <Loader2Icon className="animate-spin" />}
                                        Use current position
                                    </Button>
                                    <FormField
                                        control={form.control}
                                        name="radius"
                                        render={({ field: { value, onChange } }) => (
                                            <FormItem>
                                                <FormLabel>Radius - {value} [km]</FormLabel>
                                                <FormControl>
                                                    <Slider
                                                        min={1}
                                                        max={100}
                                                        step={1}
                                                        defaultValue={[value]}
                                                        onValueChange={(vals) => {
                                                            onChange(vals[0]);
                                                        }}
                                                        value={[form.getValues('radius')]}
                                                    />
                                                </FormControl>
                                                <FormMessage />
                                            </FormItem>
                                        )}
                                    />
                                </div>
                            </TabsContent>
                            <TabsContent value="map">
                                <div className="h-[500px] w-[600px]">
                                    <Map showDraw={true} />
                                </div>
                            </TabsContent>
                        </Tabs>
                        <DialogFooter>
                            <Button
                                type="submit"
                                onClick={() => {
                                    setSelectedTab('information');
                                }}
                            >
                                Save changes
                            </Button>
                        </DialogFooter>
                    </form>
                </Form>
            </DialogContent>
        </Dialog>
    );
}

export default LocationModal;
