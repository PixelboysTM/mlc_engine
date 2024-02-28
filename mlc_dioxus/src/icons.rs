use dioxus::prelude::*;

#[component]
pub fn Settings(cx: Scope) -> Element {
    let size = "1.25rem";
    cx.render(
        rsx! {
            svg {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                height: "{size}",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                width: "{size}",
                fill: "none",
                stroke_width: "2",
                class: "lucide lucide-settings",
                path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                circle { cy: "12", r: "3", cx: "12" }
            }
    })
}

#[component]
pub fn Pencil(cx: Scope) -> Element {
    let size = "1.25rem";
    cx.render(rsx! {
        svg {
            stroke_width: "2",
            xmlns: "http://www.w3.org/2000/svg",
            stroke_linejoin: "round",
            fill: "none",
            width: "{size}",
            stroke: "currentColor",
            stroke_linecap: "round",
            view_box: "0 0 24 24",
            height: "{size}",
            class: "lucide lucide-pencil",
            path { d: "M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" }
            path { d: "m15 5 4 4" }
        }
    })
}

#[component]
pub fn LightBulb(cx: Scope) -> Element {
    let size = "1.25rem";
    cx.render(rsx! {
        svg {
            fill: "none",
            view_box: "0 0 24 24",
            height: "{size}",
            stroke_width: "2",
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}",
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
pub fn Save(cx: Scope) -> Element {
    let size = "1.25rem";
    cx.render(rsx! {
         svg {
        height: "{size}",
        stroke_linecap: "round",
        width: "{size}",
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
pub fn UploadCloud(cx: Scope) -> Element {
    let size = "1.25rem";
    cx.render(rsx! {
            svg {
        stroke_linejoin: "round",
        fill: "none",
        stroke_linecap: "round",
        height: "{size}",
        view_box: "0 0 24 24",
        width: "{size}",
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
pub fn ExternalLink(cx: Scope) -> Element {
    let size = "1.25rem";
    cx.render(rsx! { svg {
            stroke_linecap: "round",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_linejoin: "round",
            width: "{size}",
            fill: "none",
            stroke_width: "2",
            height: "{size}",
            class: "lucide lucide-external-link",
            path { d: "M15 3h6v6" }
            path { d: "M10 14 21 3" }
            path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
        }
    })
}

#[component]
pub fn TabletSmartphone(cx: Scope) -> Element {
    let size = "1.25rem";
    cx.render(rsx! {svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke_linecap: "round",
        height: "24",
        stroke_linejoin: "round",
        stroke_width: "2",
        width: "24",
        stroke: "currentColor",
        xmlns: "http://www.w3.org/2000/svg",
        class: "lucide lucide-tablet-smartphone",
        rect { width: "10", rx: "2", height: "14", y: "8", x: "3" }
        path { d: "M5 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2h-2.4" }
        path { d: "M8 18h.01" }
    }

    })
}
