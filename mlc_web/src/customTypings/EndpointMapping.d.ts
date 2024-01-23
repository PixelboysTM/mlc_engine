
export interface EndpointMappingConfig {
    endpoints: Map<string, EndpointConfig[]>,
}

export type EndpointConfig = "Logger" | "ArtNet" | Sacn;

export type Sacn = {"Sacn" : {"universe": number, "speed": Speed}}

export type Speed = "Slow" | "Medium" | "Fast";