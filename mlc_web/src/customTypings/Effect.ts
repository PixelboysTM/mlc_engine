import {writable} from "svelte/store";


export let openEffect = writable("");

export type Effect = {
    id: string;
    name: string;
    looping: boolean;
    duration: number;
    tracks: Track[];
}

export type Track = {
    "FaderTrack": FaderTrack
};

export type FaderTrack = {
    address: FaderAddress;
    values: FaderKey[];
};

export type FaderAddress = {
    universe: number;
    address: number;
};

export type FaderKey = {
    value: number;
    start_time: number;
}