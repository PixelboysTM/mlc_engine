#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    width: Option<String>,
    height: Option<String>,
}

pub fn Settings(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());

    rsx! {
            svg {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                height: width,
                width: height,
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                fill: "none",
                stroke_width: "2",
                class: "lucide lucide-settings",
                path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                circle { cy: "12", r: "3", cx: "12" }
            }
    }
}

pub fn Pencil(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! {
        svg {
            stroke_width: "2",
            xmlns: "http://www.w3.org/2000/svg",
            stroke_linejoin: "round",
            fill: "none",
            width: width,
            height: height,
            stroke: "currentColor",
            stroke_linecap: "round",
            view_box: "0 0 24 24",
            class: "lucide lucide-pencil",
            path { d: "M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" }
            path { d: "m15 5 4 4" }
        }
    }
}

pub fn LightBulb(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! {
        svg {
            fill: "none",
            view_box: "0 0 24 24",
            width: width,
            height: height,
            stroke_width: "2",
            xmlns: "http://www.w3.org/2000/svg",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "lucide lucide-lightbulb",
            path { d: "M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5" }
            path { d: "M9 18h6" }
            path { d: "M10 22h4" }
        }
    }
}

pub fn Save(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! {
         svg {
        width: width,
        height: height,
        stroke_linecap: "round",
        xmlns: "http://www.w3.org/2000/svg",
        view_box: "0 0 24 24",
        fill: "none",
        stroke_linejoin: "round",
        stroke_width: "2",
        stroke: "currentColor",
        class: "lucide lucide-save",
        path { d: "M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" }
        polyline { points: "17 21 17 13 7 13 7 21" }
        polyline { points: "7 3 7 8 15 8" }
    }

    }
}

pub fn UploadCloud(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! {
            svg {
        stroke_linejoin: "round",
        fill: "none",
        stroke_linecap: "round",
        width: width,
        height: height,
        view_box: "0 0 24 24",
        stroke: "currentColor",
        stroke_width: "2",
        xmlns: "http://www.w3.org/2000/svg",
        class: "lucide lucide-upload-cloud",
        path { d: "M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" }
        path { d: "M12 12v9" }
        path { d: "m16 16-4-4-4 4" }
    }
        }
}

pub fn ExternalLink(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
            stroke_linecap: "round",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_linejoin: "round",
            width: width,
            height: height,
            fill: "none",
            stroke_width: "2",
            class: "lucide lucide-external-link",
            path { d: "M15 3h6v6" }
            path { d: "M10 14 21 3" }
            path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
        }
    }
}

pub fn TabletSmartphone(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! {svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke_linecap: "round",
        width: width,
        height: height,
        stroke_linejoin: "round",
        stroke_width: "2",
        stroke: "currentColor",
        xmlns: "http://www.w3.org/2000/svg",
        class: "lucide lucide-tablet-smartphone",
        rect { width: "10", rx: "2", height: "14", y: "8", x: "3" }
        path { d: "M5 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2h-2.4" }
        path { d: "M8 18h.01" }
    }

    }
}

pub fn Cable(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! {svg {
        fill: "none",
        stroke_linejoin: "round",
        stroke: "currentColor",
        width: width,
        height: height,
        stroke_width: "2",
        xmlns: "http://www.w3.org/2000/svg",
        stroke_linecap: "round",
        view_box: "0 0 24 24",
        class: "lucide lucide-cable",
        path { d: "M4 9a2 2 0 0 1-2-2V5h6v2a2 2 0 0 1-2 2Z" }
        path { d: "M3 5V3" }
        path { d: "M7 5V3" }
        path { d: "M19 15V6.5a3.5 3.5 0 0 0-7 0v11a3.5 3.5 0 0 1-7 0V9" }
        path { d: "M17 21v-2" }
        path { d: "M21 21v-2" }
        path { d: "M22 19h-6v-2a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2Z" }
    }

    }
}

pub fn Download(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        stroke: "currentColor",
        xmlns: "http://www.w3.org/2000/svg",
        stroke_width: "2",
        stroke_linecap: "round",
        width: width,
        height: height,
        view_box: "0 0 24 24",
        fill: "none",
        stroke_linejoin: "round",
        class: "lucide lucide-download",
        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
        polyline { points: "7 10 12 15 17 10" }
        line { y2: "3", x1: "12", x2: "12", y1: "15" }
    }


    }
}

pub fn FileUp(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        width: width,
        height: height,
        stroke_linecap: "round",
        stroke_width: "2",
        view_box: "0 0 24 24",
        xmlns: "http://www.w3.org/2000/svg",
        stroke: "#ffffff",
        stroke_linejoin: "round",
        fill: "none",
        class: "lucide lucide-file-up",
        path { d: "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" }
        path { d: "M14 2v4a2 2 0 0 0 2 2h4" }
        path { d: "M12 12v6" }
        path { d: "m15 15-3-3-3 3" }
    }}
}

pub fn Plus(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        width: width,
        height: height,
        xmlns: "http://www.w3.org/2000/svg",
        stroke_linejoin: "round",
        view_box: "0 0 24 24",
        class: "lucide lucide-plus",
        path { d: "M5 12h14" }
        path { d: "M12 5v14" }
    }}
}

pub fn FolderOpen(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        width: width,
        height: height,
        fill: "none",
        stroke_width: "2",
        stroke_linejoin: "round",
        stroke: "#ffffff",
        stroke_linecap: "round",
        xmlns: "http://www.w3.org/2000/svg",
        view_box: "0 0 24 24",
        class: "lucide lucide-folder-open",
        path { d: "m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2" }
    }}
}

pub fn X(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
            view_box: "0 0 24 24",
            width: width,
            height: height,
            stroke: "currentColor",
            stroke_linejoin: "round",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            stroke_width: "2",
            stroke_linecap: "round",
            class: "lucide lucide-x",
            path { d: "M18 6 6 18" }
            path { d: "m6 6 12 12" }
        }
    }
}

pub fn Lamp(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
            stroke_linecap: "round",
            width: width,
            height: height,
            view_box: "0 0 24 24",
            fill: "none",
            stroke_width: "2",
            xmlns: "http://www.w3.org/2000/svg",
            stroke: "currentColor",
            stroke_linejoin: "round",
            class: "lucide lucide-lamp",
            path { d: "M8 2h8l4 10H4L8 2Z" }
            path { d: "M12 12v6" }
            path { d: "M8 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H8Z" }
        }

    }
}

pub fn LampDesk(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
            width: width,
            height: height,
            stroke_linecap: "round",
            fill: "none",
            stroke_width: "2",
            stroke: "currentColor",
            stroke_linejoin: "round",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            class: "lucide lucide-lamp-desk",
            path { d: "m14 5-3 3 2 7 8-8-7-2Z" }
            path { d: "m14 5-3 3-3-3 3-3 3 3Z" }
            path { d: "M9.5 6.5 4 12l3 6" }
            path { d: "M3 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H3Z" }
        }
    }
}

pub fn Blocks(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        xmlns: "http://www.w3.org/2000/svg",
        width: width,
        height: height,
        stroke_linejoin: "round",
        stroke: "currentColor",
        view_box: "0 0 24 24",
        fill: "none",
        stroke_width: "2",
        stroke_linecap: "round",
        class: "lucide lucide-blocks",
        rect { rx: "1", width: "7", height: "7", x: "14", y: "3" }
        path { d: "M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3" }
    }

    }
}

pub fn Minus(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        fill: "none",
        width: width,
        height: height,
        xmlns: "http://www.w3.org/2000/svg",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linejoin: "round",
        stroke_linecap: "round",
        view_box: "0 0 24 24",
        class: "lucide lucide-minus",
        path { d: "M5 12h14" }
    }


    }
}

pub fn Check(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        width: width,
        height: height,
        view_box: "0 0 24 24",
        stroke_width: "2",
        stroke_linecap: "round",
        fill: "none",
        stroke: "currentColor",
        xmlns: "http://www.w3.org/2000/svg",
        stroke_linejoin: "round",
        class: "lucide lucide-check",
        path { d: "M20 6 9 17l-5-5" }
    }
    }
}

pub fn FileArchive(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        width,
        height,
        "viewBox": "0 0 24 24",
        "stroke-linejoin": "round",
        "stroke-width": "2",
        "stroke-linecap": "round",
        "xmlns": "http://www.w3.org/2000/svg",
        "fill": "none",
        "stroke": "currentColor",
        class: "lucide lucide-file-archive",
        path { "d": "M16 22h2a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v18" }
        path { "d": "M14 2v4a2 2 0 0 0 2 2h4" }
        circle { "r": "2", "cx": "10", "cy": "20" }
        path { "d": "M10 7V6" }
        path { "d": "M10 12v-1" }
        path { "d": "M10 18v-2" }
    }
    }
}

pub fn FileJson(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        "stroke": "currentColor",
        "stroke-linecap": "round",
        "fill": "none",
        "stroke-linejoin": "round",
        width,
        height,
        "stroke-width": "2",
        "viewBox": "0 0 24 24",
        "xmlns": "http://www.w3.org/2000/svg",
        class: "lucide lucide-file-json",
        path { "d": "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" }
        path { "d": "M14 2v4a2 2 0 0 0 2 2h4" }
        path { "d": "M10 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1" }
        path { "d": "M14 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1" }
    }
    }
}

pub fn PencilRuler(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        "xmlns": "http://www.w3.org/2000/svg",
        width,
        height,
        "viewBox": "0 0 24 24",
        "stroke-linejoin": "round",
        "stroke-width": "2",
        "stroke": "currentColor",
        "fill": "none",
        "stroke-linecap": "round",
        class: "lucide lucide-pencil-ruler",
        path { "d": "m15 5 4 4" }
        path { "d": "M13 7 8.7 2.7a2.41 2.41 0 0 0-3.4 0L2.7 5.3a2.41 2.41 0 0 0 0 3.4L7 13" }
        path { "d": "m8 6 2-2" }
        path { "d": "m2 22 5.5-1.5L21.17 6.83a2.82 2.82 0 0 0-4-4L3.5 16.5Z" }
        path { "d": "m18 16 2-2" }
        path { "d": "m17 11 4.3 4.3c.94.94.94 2.46 0 3.4l-2.6 2.6c-.94.94-2.46.94-3.4 0L11 17" }
    }

    }
}

pub fn MessageCircleQuestion(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        "xmlns": "http://www.w3.org/2000/svg",
        width,
        height,
        "fill": "none",
        "stroke-linejoin": "round",
        "stroke-linecap": "round",
        "stroke-width": "2",
        "viewBox": "0 0 24 24",
        "stroke": "currentColor",
        class: "lucide lucide-message-circle-question",
        path { "d": "M7.9 20A9 9 0 1 0 4 16.1L2 22Z" }
        path { "d": "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" }
        path { "d": "M12 17h.01" }
    }
    }
}

pub fn Palette(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        "xmlns": "http://www.w3.org/2000/svg",
        width,
        height,
        "stroke-linejoin": "round",
        "stroke": "currentColor",
        "fill": "none",
        "stroke-width": "2",
        "stroke-linecap": "round",
        "viewBox": "0 0 24 24",
        class: "lucide lucide-palette",
        circle { "cy": "6.5", "r": ".5", "cx": "13.5", "fill": "currentColor" }
        circle { "cy": "10.5", "r": ".5", "fill": "currentColor", "cx": "17.5" }
        circle { "fill": "currentColor", "r": ".5", "cx": "8.5", "cy": "7.5" }
        circle { "r": ".5", "fill": "currentColor", "cy": "12.5", "cx": "6.5" }
        path { "d": "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" }
    }
    }
}

pub fn Wand(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        "xmlns": "http://www.w3.org/2000/svg",
        width,
        height,
        "stroke": "currentColor",
        "stroke-width": "2",
        "stroke-linejoin": "round",
        "stroke-linecap": "round",
        "fill": "none",
        "viewBox": "0 0 24 24",
        class: "lucide lucide-wand",
        path { "d": "M15 4V2" }
        path { "d": "M15 16v-2" }
        path { "d": "M8 9h2" }
        path { "d": "M20 9h2" }
        path { "d": "M17.8 11.8 19 13" }
        path { "d": "M15 9h0" }
        path { "d": "M17.8 6.2 19 5" }
        path { "d": "m3 21 9-9" }
        path { "d": "M12.2 6.2 11 5" }
    }
    }
}

pub fn Trash2(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        "stroke-linejoin": "round",
        "viewBox": "0 0 24 24",
        "stroke-width": "2",
        "stroke-linecap": "round",
        "stroke": "currentColor",
        "xmlns": "http://www.w3.org/2000/svg",
        width,
        height,
        "fill": "none",
        class: "lucide lucide-trash-2",
        path { "d": "M3 6h18" }
        path { "d": "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
        path { "d": "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
        line { "x1": "10", "y1": "11", "y2": "17", "x2": "10" }
        line { "x1": "14", "x2": "14", "y1": "11", "y2": "17" }
    }
    }
}

pub fn PanelLeftClose(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        "fill": "none",
        "stroke-linecap": "round",
        width,
        height,
        "viewBox": "0 0 24 24",
        "stroke-width": "2",
        "xmlns": "http://www.w3.org/2000/svg",
        "stroke-linejoin": "round",
        "stroke": "currentColor",
        class: "lucide lucide-panel-left-close",
        rect { width: "18", "x": "3", "y": "3", height: "18", "rx": "2" }
        path { "d": "M9 3v18" }
        path { "d": "m16 15-3-3 3-3" }
    }

    }
}

pub fn PanelLeftOpen(props: IconProps) -> Element {
    let width = props.width.unwrap_or("1.25rem".to_string());
    let height = props.height.unwrap_or("1.25rem".to_string());
    rsx! { svg {
        "stroke-linecap": "round",
        "stroke": "currentColor",
        "stroke-width": "2",
        width,
        height,
        "fill": "none",
        "xmlns": "http://www.w3.org/2000/svg",
        "viewBox": "0 0 24 24",
        "stroke-linejoin": "round",
        class: "lucide lucide-panel-left-open",
        rect { height: "18", width: "18", "y": "3", "rx": "2", "x": "3" }
        path { "d": "M9 3v18" }
        path { "d": "m14 9 3 3-3 3" }
    }


    }
}
