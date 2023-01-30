use yew::{classes, html, Component, Context, Html};

pub struct Login {
    username: String,
    password: String,
}

pub enum LoginMsg {
    UpdateUsername(String),
    UpdatePassword(String),
    Submit,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::UpdateUsername(username) => self.username = username,
            LoginMsg::UpdatePassword(password) => self.password = password,
            LoginMsg::Submit => {
                // TODO: Send the data to the server and handle the response
            }
        }
        true
    }
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|_| LoginMsg::Submit);
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
                <button {onsubmit} type="submit">{"Submit"}</button>
            </form>
            </>
        }
    }
}
