#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use assets::files::*;
use db::queries::{datasets::Dataset, models::Model};
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
pub fn Page(
    cx: Scope,
    organisation_id: i32,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
) -> Element {
    cx.render(rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Datasets,
            team_id: *organisation_id,
            title: "Datasets",
            header: cx.render(rsx!(
                h3 { "Datasets" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "new-dataset-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Dataset"
                }
            )),

            if datasets.is_empty() {
                cx.render(rsx! {
                    BlankSlate {
                        heading: "Looks like you don't have any datasets yet",
                        visual: nav_ccsds_data_svg.name,
                        description: "Datasets allow you to organize your documents like folders"
                    }
                })
            } else {
                cx.render(rsx! {
                    Box {
                        class: "has-data-table",
                        BoxHeader {
                            title: "Datasets"
                        }
                        BoxBody {
                            table {
                                class: "table table-sm",
                                thead {
                                    th { "Name" }
                                    th { "Visibility" }
                                    th { "Document Count" }
                                    th { "Chunking Strategy" }
                                    th {
                                        class: "text-right",
                                        "Action"
                                    }
                                }
                                tbody {

                                    datasets.iter().map(|dataset| {
                                        cx.render(rsx!(
                                            tr {
                                                td {
                                                    a {
                                                        href: "{crate::routes::documents::index_route(*organisation_id, dataset.id)}",
                                                        "{dataset.name}" 
                                                    }
                                                }
                                                td {
                                                    crate::prompts::visibility::VisLabel {
                                                        visibility: dataset.visibility
                                                    }
                                                }
                                                td { "{dataset.count}" }
                                                td {
                                                    Label {
                                                        label_role: LabelRole::Highlight,
                                                        "By Title"
                                                    }
                                                    }
                                                td {
                                                    class: "text-right",
                                                    DropDown {
                                                        direction: Direction::Left,
                                                        button_text: "...",
                                                        DropDownLink {
                                                            href: "{crate::routes::documents::index_route(*organisation_id, dataset.id)}",
                                                            target: "_top",
                                                            "View"
                                                        }
                                                    }
                                                }
                                            }
                                        ))
                                    })
                                }
                            }
                        }
                    }
                })
            }

            super::new::New {
                models: models.clone(),
                organisation_id: *organisation_id,
                combine_under_n_chars: 500,
                new_after_n_chars: 1000,
                _multipage_sections: true,
            }
        }
    })
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
