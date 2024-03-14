use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct IconProps<'a> {
    width: Option<&'a str>,
    height: Option<&'a str>,
}

#[component]
pub fn Settings<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(
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
    })
}

#[component]
pub fn Pencil<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! {
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
    })
}

#[component]
pub fn LightBulb<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! {
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
    })
}

#[component]
pub fn Save<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! {
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

    })
}

#[component]
pub fn UploadCloud<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! {
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
        })
}

#[component]
pub fn ExternalLink<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! { svg {
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
    })
}

#[component]
pub fn TabletSmartphone<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! {svg {
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

    })
}

#[component]
pub fn Cable<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! {svg {
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

    })
}

#[component]
pub fn Download<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! { svg {
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


    })
}

#[component]
pub fn FileUp<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! { svg {
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
    }})
}

#[component]
pub fn Plus<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! { svg {
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
    }})
}

#[component]
pub fn FolderOpen<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! { svg {
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
    }})
}

#[component]
pub fn X<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! { svg {
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
    })
}

#[component]
pub fn Lamp<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! { svg {
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

    })
}

#[component]
pub fn LampDesk<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let width = cx.props.width.unwrap_or("1.25rem");
    let height = cx.props.height.unwrap_or("1.25rem");
    cx.render(rsx! { svg {
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
    })
}
