import { Button } from '@/components/ui/button';
import useMapStore, { CircleCoordinates, RectangleCoordinates } from '@/lib/store/map';
import L from 'leaflet';
import React, { useEffect, useState } from 'react';
import DrawCircle from './DrawCircle/DrawCircle';
import DrawRectangle from './DrawRectangle';

function Draw() {
    const mode = useMapStore((state) => state.mode);
    const enabled = useMapStore((state) => state.draw);
    const stopDraw = useMapStore((state) => state.stopDraw);
    const predefinedCoordinates = useMapStore((state) => state.coordinates);
    const [coordinates, setCoordinates] = useState<RectangleCoordinates | CircleCoordinates | null>(null);

    useEffect(() => {
        return () => {
            stopDraw();
        };
    }, []);

    const stopPropagation = (el: HTMLButtonElement | null) => {
        if (el === null) {
            return;
        }

        L.DomEvent.disableClickPropagation(el);
        L.DomEvent.disableScrollPropagation(el);
    };

    const handleCoordinatesChange = (coordinates: RectangleCoordinates | CircleCoordinates | null) => {
        setCoordinates(coordinates);
    };

    const handleCancelClick = () => {
        stopDraw();
    };

    const handleConfirmClick = () => {
        if (!coordinates) {
            stopDraw();
        }

        stopDraw(coordinates as RectangleCoordinates | CircleCoordinates);
    };

    const renderPanel = () => {
        return (
            <div className="z-[1000] absolute bottom-6 w-full flex justify-center gap-2">
                <Button
                    variant="outline"
                    className="shadow-lg"
                    onClick={handleCancelClick}
                    ref={stopPropagation}
                >
                    Cancel
                </Button>
                <Button
                    className="shadow-lg"
                    onClick={handleConfirmClick}
                    ref={stopPropagation}
                    disabled={!coordinates}
                >
                    Confirm
                </Button>
            </div>
        );
    };

    const renderDraw = () => {
        switch (mode) {
            case 'rectangle':
                return (
                    <DrawRectangle
                        coordinates={predefinedCoordinates as RectangleCoordinates}
                        onChange={handleCoordinatesChange}
                    />
                );

            case 'circle':
                return (
                    <DrawCircle
                        coordinates={predefinedCoordinates as CircleCoordinates}
                        onChange={handleCoordinatesChange}
                    />
                );
        }
    };

    if (!enabled) {
        return null;
    }

    return (
        <React.Fragment>
            {renderPanel()}
            {renderDraw()}
        </React.Fragment>
    );
}

export default Draw;
