#![recursion_limit = "256"]

extern crate serde;

use anyhow;
use self::serde::{Deserialize};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct Model {
    link: ComponentLink<Self>,
    rows: Vec<Row>,
    loaded: bool
}

struct Row {
    class: String,
    name: String,
    days: Option<String>,
    time: Option<String>,
    credits: i32,
    instructor: String,
    rating: Option<i32>,
}

pub enum Msg {
    More,
    Less,
    FetchResourceComplete,
    FetchResourceFailed,
    NoOp
}

#[derive(Deserialize)]
pub struct Term {
    pub date: i32,
    pub description: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TODO: fetch data from back-end
//        let get_request = Request::get("http://localhost:8080/api/terms").body(Nothing).unwrap();
//        let callback = link.callback(|response: Response<Json<Result<Term, anyhow::Error>>>| {
//            if let (meta, Json(Ok(body))) = response.into_parts() {
//                if meta.status.is_success() {
//                    println!("success!");
//                    return Msg::FetchResourceComplete;
//                }
//            }
//            println!("failure!");
//            Msg::FetchResourceFailed
//        });
//        let task = FetchService::new().fetch(get_request, callback);
//        println!( "task: {:?}", task);

        let row = Row {
            class: "class".to_string(),
            name: "name".to_string(),
            days: Some("days".to_string()),
            time: Some("time".to_string()),
            credits: 4,
            instructor: "instructor".to_string(),
            rating: Some(32),
        };

        let row2 = Row {
            class: "class".to_string(),
            name: "name".to_string(),
            days: Some("days".to_string()),
            time: None,
            credits: 4,
            instructor: "instructor".to_string(),
            rating: Some(32),
        };

        let rows = vec![row, row2];

        Model { link, rows, loaded: true }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchResourceComplete => {
                self.loaded = true;
            }
            Msg::FetchResourceFailed => {
                self.loaded = false;
            }
            _ => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{self.loaded}</h1>
                <table>
                    <thead>
                        {self.view_headers()}
                    </thead>
                    <tbody>
                        { for self.rows.iter().map(|r| self.view_row(r)) }
                    </tbody>
                </table>
            </>
        }
    }
}

impl Model {
    fn view_headers(&self) -> Html {
        let headers = [ "Class", "Name", "Days", "Time", "Credits", "Instructor", "Rating"];
        html! {
            <tr>
                { for headers.iter().map(|h| html! { <th>{h}</th> }) }
            </tr>
        }
    }

    fn view_row(&self, row: &Row) -> Html {
        html! {
            <tr>
                <td>{row.class.to_string()}</td>
                <td>{row.name.to_string()}</td>
                <td>{row.days.as_ref().unwrap()}</td>
                <td>{row.time.as_ref().unwrap_or(&"".to_string())}</td>
                <td>{row.credits.to_string()}</td>
                <td>{row.instructor.to_string()}</td>
                <td>{row.rating.unwrap()}</td>
            </tr>
        }
    }

    fn fetch_terms_json(&mut self) -> yew::services::fetch::FetchTask {
        let callback = self.link.callback(
            move |response: Response<Json<Result<Term, anyhow::Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}", meta);
                if meta.status.is_success() {
//                    Msg::FetchReady(data)
                    Msg::FetchResourceComplete
                } else {
                    Msg::NoOp // FIXME: Handle this error accordingly.
                }
            },
        );
        let request = Request::get("http://localhost:8080/api/terms").body(Nothing).unwrap();
        let mut fetch_service = FetchService::new();

        fetch_service.fetch(request, callback)
    }
}
