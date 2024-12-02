use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {}

#[derive(Template)]
#[template(path = "pages/todo.html")]
pub struct TodoTemplate {}

#[derive(Template)]
#[template(path = "pages/create.html")]
pub struct CreateTemplate {}

#[derive(Template)]
#[template(path = "pages/sign_in.html")]
pub struct SignInTemplate {}

#[derive(Template)]
#[template(path = "pages/sign_up.html")]
pub struct SignUpTemplate {}

#[derive(Template)]
#[template(path = "pages/server_error.html")]
pub struct ServerErrorTemplate {}