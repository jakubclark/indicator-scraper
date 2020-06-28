mod model;
mod scraper;

use crate::model::*;
use crate::scraper::scrape_text;
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

#[derive(Debug, Clone)]
enum Msg {
    ScrapeRequest(String),
    // Sync,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ScrapeRequest(text) => {
            model.text = text;
            log("Scraping for indicators");
            model.indicators = scrape_text(model.text.as_str());
            log("Done scraping:");
            log(&model.indicators);
        } // Msg::Sync => {
          //     log("Synchronizing with backend!");
          // }
    }
}

fn view_header() -> Node<Msg> {
    header![C!["header"], h1!["Scrape for Indicators"],]
}

// fn view_scrape_button() -> Node<Msg> {
//     button!("Synchronize with backend!", ev(Ev::Click, |_| Msg::Sync))
// }

fn view_scrape_area() -> Node<Msg> {
    div![
        C!["form-group"],
        id!("scrape-text-div"),
        textarea![
            C!["form-control"],
            id!("scrape-text-area"),
            input_ev(Ev::Input, Msg::ScrapeRequest),
        ]
    ]
}

fn view_scrape_results(indicators: &IndicatorResults) -> Node<Msg> {
    section![
        C!["scrape-results"],
        indicators.iter().map(|(k, values)| {
            IF!(!values.is_empty() => vec![
                h3![format!["{} Scrape Results", k]],
                table![
                    C!["scrape-results-table"],
                    tr![
                        th!["Indicator Value"],
                        th!["Start Position"],
                        th!["End Position"],
                    ],
                    values.iter().map(|v| {
                        tr![
                            td![v.text.as_str()],
                            td![v.start_position],
                            td![v.end_position],
                        ]
                    })
                ],
            ]
                )
        })
    ]
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        view_header(),
        view_scrape_area(),
        // br!(),
        // view_scrape_button(),
        view_scrape_results(&model.indicators),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    let root_element = document()
        .get_elements_by_class_name("scrape-app")
        .item(0)
        .expect("element with the class `scrape-app`");

    App::start(root_element, init, update, view);
}
