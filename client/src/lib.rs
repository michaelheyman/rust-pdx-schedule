#![recursion_limit = "256"]

extern crate serde;

use self::serde::Deserialize;
use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    console: ConsoleService,
    fetch_service: FetchService,
    link: ComponentLink<Self>,
    fetching: bool,
    data: Option<Course>,
    ft: Option<FetchTask>,
}

#[derive(Deserialize, Debug)]
pub struct ClassOffering {
    id: i32,
    credits: i32,
    days: Option<String>,
    time: Option<String>,
    crn: i32,
    timestamp: Option<String>,
    course: Course,
    instructor: Instructor,
    term: Term,
}

#[derive(Deserialize, Debug)]
pub struct Course {
    id: i32,
    name: String,
    number: String,
    discipline: String,
}

#[derive(Deserialize, Debug)]
pub struct Instructor {
    id: i32,
    full_name: String,
    first_name: Option<String>,
    last_name: Option<String>,
    rating: Option<f64>,
    url: Option<String>,
    timestamp: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Term {
    date: i32,
    description: String,
}

pub enum Msg {
    FetchData,
    FetchReady(Result<Course, Error>),
    Ignore,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            fetch_service: FetchService::new(),
            link,
            fetching: false,
            data: None,
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.console.log("Update");
        match msg {
            Msg::FetchData => {
                self.fetching = true;
                let task = self.fetch_json();
                self.ft = Some(task);
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response
                    .map(|data| Course {
                        id: data.id,
                        name: data.name,
                        number: data.number,
                        discipline: data.discipline,
                    })
                    .ok();
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <button onclick=self.link.callback(|_| Msg::FetchData)>
                    { "Fetch Data" }
                </button>
                {self.view_course()}
                <table>
                    <thead>
                        {self.view_headers()}
                    </thead>
                    <tbody>
                    </tbody>
                </table>
            </>
        }
    }
}

impl Model {
    fn view_headers(&self) -> Html {
        let headers = [
            "Class",
            "Name",
            "Days",
            "Time",
            "Credits",
            "Instructor",
            "Rating",
        ];
        html! {
            <tr>
                { for headers.iter().map(|h| html! { <th>{h}</th> }) }
            </tr>
        }
    }

    fn view_course(&self) -> Html {
        if let Some(data) = &self.data {
            html! {
                <p>{ data.name.to_string() }</p>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }

    fn fetch_json(&mut self) -> yew::services::fetch::FetchTask {
        let callback =
            self.link
                .callback(move |response: Response<Json<Result<Course, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::FetchReady(data)
                    } else {
                        Msg::Ignore // FIXME: Handle this error accordingly.
                    }
                });
        let request = Request::get("http://localhost:8080/api/course/1")
            .body(Nothing)
            .unwrap();

        self.fetch_service.fetch(request, callback)
    }
}
