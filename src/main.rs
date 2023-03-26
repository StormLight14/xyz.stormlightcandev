use gloo::console::{self};
use gloo::timers::callback::Interval;
use yew::{html, Component, Context, Html, SubmitEvent, InputEvent};

pub enum Msg {
    UpdateTime,
    UpdateUsername(String),
    UpdateRating(String),
    UpdateReview(String),
    Submit,
}

pub struct Form {
    username: String,
    rating: String,
    review: String,
}

/* 
 
impl Component for Form {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            username: "".to_string(),
            rating: "".to_string(),
            review: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(username) => {
                self.username = username;
                true
            }
            Msg::UpdateRating(rating) => {
                self.rating = rating;
                true
            }
            Msg::UpdateReview(review) => {
                self.review = review;
                true
            }
            Msg::Submit => false,
            _ => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <form onsubmit={|e: SubmitEvent| {
                e.prevent_default();
                ()
            }}>
                <label for="username">{"Username:"}</label>
                <input type="text" id="username" name="username" value={self.username} oninput={|e: InputEvent| Msg::UpdateUsername(e.value)}/>

                <label for="rating">{"Rating:"}</label>
                <input type="text" id="rating" name="rating" value={self.rating} oninput={|e: InputEvent | Msg::UpdateRating(e.value)}/>

                <label for="review">{"Review:"}</label>
                <textarea id="text" name="review" value={self.review} oninput={|e: InputEvent| Msg::UpdateReview(e.value)}></textarea>

                <button type="submit">{"Submit"}</button>
            </form>
        }
    }
}
*/
 
pub struct App {
    time: String,
    messages: Vec<&'static str>,
    _standalone: (Interval, Interval),
}

impl App {
    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("en-GB"))
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let standalone_handle =
            Interval::new(10, || console::debug!("Example of a standalone callback."));

        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            time: App::get_current_time(),
            messages: Vec::new(),
            _standalone: (standalone_handle, clock_handle),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTime => {
                self.time = App::get_current_time();
                true
            }
            _ => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div id="wrapper">
                    <p>{"Time"}</p>
                    <div id="time">
                        { &self.time }
                    </div>
                    <div id="messages">
                        { for self.messages.iter().map(|message| html! { <p>{ message }</p> }) }
                    </div>
                </div>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
