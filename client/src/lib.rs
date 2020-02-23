#![recursion_limit = "256"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    rows: Vec<Row>,
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
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TODO: fetch data from back-end
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

        Model { link, rows }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
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
}
