declare module "fixture-types" {
    export interface FixtureUniverse {
        id: string;
        channels: Option<PatchedChannelIndex>[];
        fixtures: PatchedFixture[];
    }

    export interface PatchedChannelIndex {
        fixture_index: number;
        channel_index: number;
    }

    export interface PatchedFixture {
        config: FixtureType;
        num_channels: number;
        channels: PatchedChannel[];
        start_channel: number;
        name: string;
    }

    export interface PatchedChannel {
        config: FixtureChannel;
        channel_address: number;
    }

    export interface FixtureType {
        name: string;
        categories: string[];
        fixture_key: string;
        manufacturer: Manufacturer;
        modes: FixtureMode[];
        available_channels: Map<string, FixtureChannel>;
        id: string;
    }

    export interface FixtureMode {
        name: string;
        short_name: string;
        channels: string[];
    }

    export interface FixtureChannel {
        default_value: number;
        capabilities: FixtureCapability[];
    }

    export type FixtureCapability = any;

    export interface Manufacturer {
        name: string;
        website: string;
    }

    export type Option<T> = null | undefined | T;
}