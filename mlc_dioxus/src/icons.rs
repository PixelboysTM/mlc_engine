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
        stroke: "#ffffff",
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