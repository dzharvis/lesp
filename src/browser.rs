use yew::prelude::*;
use yew::{html, html_impl};
use crate::lisp::{Context, eval_in_context};

pub struct RootModel {
    inputs: Vec<(String, String)>
}

pub enum RootMsg {
    Eval(String), Noop
}

impl Component<Context> for RootModel {
    type Message = RootMsg;
    type Properties = ();

    fn create(_: Self::Properties, mut context: &mut Env<Context, Self>) -> Self {
        let example = String::from("(defn identity (a) a)");
        RootModel { inputs: vec![(example.clone(), format!("{:?}", eval_in_context(&example, &mut context)))] }
    }

    fn update(&mut self, msg: Self::Message, mut context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            RootMsg::Noop => false,
            RootMsg::Eval(form) => {
                let result = format!("{:?}", eval_in_context(&form, &mut context));
                self.inputs.push((form, result));
                true
            }
        }
    }
}

fn io(io: (String, String)) -> Html<Context, RootModel> {
    html! {
        <div>{io.0} {" => "} <kbd>{io.1}</kbd></div>
    }
}

impl Renderable<Context, RootModel> for RootModel {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div class={"jumbotron"},><div class={"container"},>
                <h2 class={"text-center"},>{"Lesp"}</h2>
                <pre>
                 { for self.inputs.iter().map(|i| {
                      io(i.clone())
                 })}
                </pre>
                <pre>{">> "}<input type={"text"}, placeholder={"(map (list 1 2 3) (fn _ (a) (* a a)))"}, onchange=|e| match e {
                    ChangeData::Value(se) => RootMsg::Eval(se),
                    _ => unreachable!(),
                 }, /></pre>
            <div/></div>
        }
    }
}