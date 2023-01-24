use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Comp{
    name: String,
    plus: isize,
    minus: isize,
}


#[function_component]
fn Com(arg: &Comp) -> Html{
    let c = use_state(|| 0);
    let cls = use_state(||"");
    let onclick = {
        let cls = cls.clone();
        let c = c.clone();
        let plus = arg.plus.clone();
        let minus = arg.minus.clone();
        
        move |_| {
            if *cls == "boom1"{
                cls.set("boom2");
            }else{
                cls.set("boom1");
            }
            c.set(*c + plus - minus);
        }
    };
    html!{
        <div class="Com">
            <p id="count" class={ *cls }>{ *c }</p>
            <button {onclick} id="btn">{ arg.name.clone() }</button>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    println!("hello!");
    html! {
        <div>
            <Com name="+1" plus=1 minus=0></Com>
            <Com name="-1" plus=0  minus=1></Com>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
