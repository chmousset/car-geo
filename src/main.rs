#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use std::f64::consts::PI;
use std::str::FromStr;

use dioxus_bulma::components::containers::{Box, Container, Content, Footer, Section};
use dioxus_bulma::components::form::*;
use dioxus_bulma::styles::BulmaStylesheet;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
pub fn VarMmInput(
    label: Option<String>,
    help: Option<String>,
    icon_left: Option<String>,
    icon_right: Option<String>,
    horizontal: Option<bool>,
    expanded: Option<bool>,
    value: Signal<f64>,
    placeholder: Option<String>,
    addon_left: Element,
    addon_right: Element,
) -> Element {
    let mut invalid = use_signal(|| false);
    let oninput = move |event: Event<FormData>| {
        if let Ok(parsed_value) = f64::from_str(&event.value()) {
            let mut value_clone = value.clone();
            value_clone.set(parsed_value);
            *invalid.write() = false;
        } else {
            *invalid.write() = true;
        }
        event.stop_propagation();
    };

    rsx! (
        FieldWrapper {
            label,
            help,
            icon_left,
            icon_right,
            addon_left,
            addon_right,
            horizontal,
            expanded,
            input {
                oninput,
                class: "input",
                class: if invalid() {"is-danger"},
                "type": "number",
                placeholder,
                value: value,
            }
        }
    )
}

#[component]
fn App() -> Element {
    let Wb = use_signal(|| 2420 as f64); // wheelbase
    let Fo = use_signal(|| (4.181 - 2.420) * 500.0 as f64); // front_overhang
    let Ro = use_signal(|| (4.181 - 2.420) * 500.0 as f64); // read_overhang

    let Wffl = use_signal(|| 0.0 as f64); // Front-Left wheel, laser distance on Front
    let Wfbl = use_signal(|| 0.0 as f64); // Front-Left wheel, laser distance on Back
    let Wfl = use_resource(move || async move { 0.5 * (Wffl() + Wfbl()) }); // Front-Left wheel, laser distance from center hub
    let Wffr = use_signal(|| 0.0 as f64);
    let Wfbr = use_signal(|| 0.0 as f64);
    let Wfr = use_resource(move || async move { 0.5 * (Wffr() + Wfbr()) });
    let Wbfl = use_signal(|| 0.0 as f64); // Front-Left wheel, laser distance on Front
    let Wbbl = use_signal(|| 0.0 as f64); // Front-Left wheel, laser distance on Back
    let Wbl = use_resource(move || async move { 0.5 * (Wbfl() + Wbbl()) }); // Front-Left wheel, laser distance from center hub
    let Wbfr = use_signal(|| 0.0 as f64);
    let Wbbr = use_signal(|| 0.0 as f64);
    let Wbr = use_resource(move || async move { 0.5 * (Wbfr() + Wbbr()) });

    let Wd = use_signal(|| 18.0 * 25.4 as f64); // Wheen Diameter
    let Lb = use_signal(|| 1980 as f64); // Laser width, Back
    let Lf = use_signal(|| 1980 as f64); // Laser width, Front
    let La2 = use_resource(move || async move {
        let tan = (Lf() - Lb()) / (Ro() + Wb() + Fo());
        tan.atan() * 0.5 * 180.0 / PI
    }); // laser angle, over 2

    let Rw = use_resource(move || async move {
        let ll: f64 = Lb() + (Lf() - Lb()) / (Ro() + Wb() + Fo()) * Ro();
        if Wfl().is_none() | Wfr().is_none() {
            return 0.0;
        }
        ll - Wfl().unwrap() - Wfr().unwrap()
    }); // read wheel width

    let Fw = use_resource(move || async move {
        let ll: f64 = Lb() + (Lf() - Lb()) / (Ro() + Wb() + Fo()) * Ro();
        ll - Wfl().or(Some(0.0)).unwrap() - Wfr().or(Some(0.0)).unwrap()
    }); // Front wheel width

    let Ca = use_resource(move || async move {
        let yb = (Wbr().or(Some(0.0)).unwrap() - Wbl().or(Some(0.0)).unwrap()) * 0.5;
        let yf = (Wfr().or(Some(0.0)).unwrap() - Wfl().or(Some(0.0)).unwrap()) * 0.5;
        let tan = (yf - yb) / Wb();
        tan.atan() * 180.0 / PI
    }); // Car angle

    let Tfl = use_resource(move || async move {
        let tan = (Wffl() - Wfbl()) / Wd();
        tan.atan() * 180.0 / PI - La2().or(Some(0.0)).unwrap() - Ca().or(Some(0.0)).unwrap()
    }); // Toe front Left

    let Tfr = use_resource(move || async move {
        let tan = (Wffr() - Wfbr()) / Wd();
        tan.atan() * 180.0 / PI - La2().or(Some(0.0)).unwrap() + Ca().or(Some(0.0)).unwrap()
    }); // Toe front Right

    let Tbl = use_resource(move || async move {
        let tan = (Wbfl() - Wbbl()) / Wd();
        tan.atan() * 180.0 / PI - La2().or(Some(0.0)).unwrap() - Ca().or(Some(0.0)).unwrap()
    }); // Toe back Left

    let Tbr = use_resource(move || async move {
        let tan = (Wbfr() - Wbbr()) / Wd();
        tan.atan() * 180.0 / PI - La2().or(Some(0.0)).unwrap() + Ca().or(Some(0.0)).unwrap()
    }); // Toe back Right

    // Camber
    let Hftl = use_signal(|| 0.0); // Front Left, Top distance from laser
    let Hfbl = use_signal(|| 0.0); // Front Left, Bottom distance from laser
    let Hftr = use_signal(|| 0.0); // Front Right, Top distance from laser
    let Hfbr = use_signal(|| 0.0); // Front Right, Bottom distance from laser
    let Hbtl = use_signal(|| 0.0); // Back Left, Top distance from laser
    let Hbbl = use_signal(|| 0.0); // Back Left, Bottom distance from laser
    let Hbtr = use_signal(|| 0.0); // Back Right, Top distance from laser
    let Hbbr = use_signal(|| 0.0); // Back Right, Bottom distance from laser
    let Cfl = use_resource(move || async move {
        let tan = (Hfbl() - Hftl()) / Wd();
        tan.atan() * 180.0 / PI
    }); // camber, Front Left
    let Cfr = use_resource(move || async move {
        let tan = (Hfbr() - Hftr()) / Wd();
        tan.atan() * 180.0 / PI
    }); // camber, Front Right
    let Cbl = use_resource(move || async move {
        let tan = (Hbbl() - Hbtl()) / Wd();
        tan.atan() * 180.0 / PI
    }); // camber, Back Left
    let Cbr = use_resource(move || async move {
        let tan = (Hbbr() - Hbtr()) / Wd();
        tan.atan() * 180.0 / PI
    }); // camber, Back Right

    rsx! {
        BulmaStylesheet {}
        link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/MaterialDesign-Webfont/7.4.47/css/materialdesignicons.min.css"
        }
        Section {
            Container {
                Box {
                    Content {
                        VarMmInput {
                            label: "Wheel Base",
                            horizontal: true,
                            value: Wb,
                            addon_left: None,
                            addon_right: None,
                        }
                        VarMmInput {
                            label: "Front Overhang",
                            horizontal: true,
                            value: Fo,
                            addon_left: None,
                            addon_right: None,
                        }
                        VarMmInput {
                            label: "Rear Overhang",
                            horizontal: true,
                            value: Ro,
                            addon_left: None,
                            addon_right: None,
                        }
                        VarMmInput {
                            label: "Front Laser width",
                            horizontal: true,
                            value: Lf,
                            addon_left: None,
                            addon_right: None,
                        }
                        VarMmInput {
                            label: "Rear Laser Width",
                            horizontal: true,
                            value: Lb,
                            addon_left: None,
                            addon_right: None,
                        }
                    }
                }
                Box {
                    table {
                        class: "table is-fullwidth is-narrow",
                        thead {
                            th {
                                colspan: 4,
                                text_align: "center",
                                "Enter distance to laser, front to back"
                            }
                        }
                        tbody {
                            tr {
                                td {
                                    VarMmInput {
                                        value: Wffl,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                                td {
                                    "Toe Left: {Tfl().or(Some(0.0)).unwrap():.2}°"
                                }
                                td {
                                    "Toe Right: {Tfr().or(Some(0.0)).unwrap():.2}°"
                                }
                                td {
                                    VarMmInput {
                                        value: Wffr,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                            }
                            tr {
                                td {"{Wfl().or(Some(0.0)).unwrap():.2}"}
                                td {
                                    colspan: 2,
                                    "Front Width: {Fw().or(Some(0.0)).unwrap():.2}"
                                }
                                td {"{Wfr().or(Some(0.0)).unwrap():.2}"}
                            }
                            tr {

                                td {
                                    VarMmInput {
                                        value: Wfbl,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                                td {
                                    colspan: 2,
                                    "Total Toe: {Tfl().or(Some(0.0)).unwrap() + Tfr().or(Some(0.0)).unwrap():.2}°"
                                }
                                td {
                                    VarMmInput {
                                        value: Wfbr,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                            }
                            tr {
                                td {
                                    colspan: 4,
                                    "Car angle: {Ca().or(Some(0.0)).unwrap():.2}°"
                                }
                            }

                            tr {
                                td {
                                    VarMmInput {
                                        value: Wbfl,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                                td {
                                    "Toe Left: {Tbl().or(Some(0.0)).unwrap():.2}°"
                                }
                                td {
                                    "Toe Right: {Tbr().or(Some(0.0)).unwrap():.2}°"
                                }
                                td {
                                    VarMmInput {
                                        value: Wbfr,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                            }
                            tr {
                                td {"{Wbl().or(Some(0.0)).unwrap():.2}"}
                                td {
                                    colspan: 2,
                                    "Width: {Rw().or(Some(0.0)).unwrap():.2}"
                                }
                                td {"{Wbr().or(Some(0.0)).unwrap():.2}"}
                            }
                            tr {
                                td {
                                    VarMmInput {
                                        value: Wbbl,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                                td {
                                    colspan: 2,
                                    "Total Toe: {Tbl().or(Some(0.0)).unwrap() + Tbr().or(Some(0.0)).unwrap():.2}°"
                                }
                                td {
                                    VarMmInput {
                                        value: Wbbr,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                            }
                        }
                    }
                }

                Box {
                    table {
                        class: "table is-fullwidth is-narrow",
                        thead {
                            th {
                                colspan: 4,
                                text_align: "center",
                                "Enter distance to laser, Top to Bottom"
                            }
                        }
                        tbody {
                            tr {
                                td {
                                    VarMmInput {
                                        value: Hftl,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                                td {
                                    "camber Left: {Cfl().or(Some(0.0)).unwrap():.2}°"
                                }
                                td {
                                    "Camber Right: {Cfr().or(Some(0.0)).unwrap():.2}°"
                                }
                                td {
                                    VarMmInput {
                                        value: Hftr,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                            }
                            tr {
                                td {
                                    VarMmInput {
                                        value: Hfbl,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                                td {
                                    colspan: 2,
                                    ""
                                }
                                td {
                                    VarMmInput {
                                        value: Hfbr,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                            }
                            tr {
                                td {
                                    colspan: 4,
                                    ""
                                }
                            }

                            tr {
                                td {
                                    VarMmInput {
                                        value: Hbtl,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                                td {
                                    "Camber Left: {Cbl().or(Some(0.0)).unwrap():.2}°"
                                }
                                td {
                                    "Camber Right: {Cbr().or(Some(0.0)).unwrap():.2}°"
                                }
                                td {
                                    VarMmInput {
                                        value: Hbtr,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                            }
                            tr {
                                td {
                                    VarMmInput {
                                        value: Hbbl,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                                td {
                                    colspan: 2,
                                    ""
                                }
                                td {
                                    VarMmInput {
                                        value: Hbbr,
                                        addon_left: None,
                                        addon_right: None,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Footer {
            div {
                class: "has-text-centered",
                p {
                    "Simple Car Geometry 0.1"
                }
                p {
                    "Copyright 2024 Charles-Henri Mousset"
                }
            }
        }
    }
}
