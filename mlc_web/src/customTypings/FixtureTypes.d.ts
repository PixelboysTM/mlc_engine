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
    }

    export type Option<T> = undefined | T;
}