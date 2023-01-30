use yew::{classes, html, Component, Context, Html};

pub struct Login;

impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <form>
                <div>
                    <label for="username">{ "Username:" }</label>
                    <input type="text" id="username" name="username"/>
                </div>
                <div>
                    <label for="password">{ "Password:" }</label>
                    <input type="password" id="password" name="password"/>
                </div>
                <button type="submit">{"Submit"}</button>
            </form>
            </>
        }
    }
}
