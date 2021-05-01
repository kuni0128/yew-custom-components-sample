mod counter;
mod button;
mod barrier;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use counter::{Color, Counter};
use barrier::Barrier;

pub struct Model {
    link: ComponentLink<Self>,
    with_barrier: bool,
    color: Color,
}

pub enum Msg {
    Repaint,
    Toggle,
    ChildClicked(u32),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            with_barrier: false,
            color: Color::Red,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Repaint => {
                self.color = Color::Blue;
                true
            }
            Msg::Toggle => {
                self.with_barrier = !self.with_barrier;
                true
            }
            Msg::ChildClicked(_value) => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let counter = |x| {
            html! {
                <Counter initial=x color=&self.color onclick=self.link.callback(Msg::ChildClicked) />
            }
        };
        html! {
            <div class="custom-components-sample">
                <button onclick=self.link.callback(|_| Msg::Toggle)>{ "Toggle" }</button>
                { self.view_barrier() }
                { for (1..1001).map(counter) }
            </div>
        }
    }
}

impl Model {
    fn view_barrier(&self) -> Html {
        if self.with_barrier {
            html! {
                <Barrier limit=10 onsignal=self.link.callback(|_| Msg::Repaint) />
            }
        } else {
            html! {
                <p>{ "Click \"toggle\"!" }</p>
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
