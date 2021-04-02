#![feature(decl_macro)]
use std::error::Error;
use rocket::{form::Form, response::Redirect, *};
use rocket_auth::{Auth, Login, Signup, User, Users};
use rocket_contrib::templates::{Template};
use serde_json::json;
type Result<T, E= Box<dyn Error>> = std::result::Result<T, E>;


#[get("/login")]
fn get_login() -> Template {
    Template::render("login", json!({}))
}

#[post("/login", data = "<form>")]
async fn post_login<'a>(mut auth: Auth<'a>, form: Form<Login>) -> Redirect {
    auth.login(&form).await.unwrap();
    Redirect::to("/")
}

#[get("/signup")]
fn get_signup() -> Template {
    Template::render("signup", json!({}))
}

#[post("/signup", data = "<form>")]
async fn post_signup<'a>(mut auth: Auth<'a>, form: Form<Signup>) -> Redirect {
    auth.signup(&form).await.unwrap();
    auth.login(&form.into()).await.unwrap();
    Redirect::to("/")
}

#[get("/")]
fn index(user: Option<User>) -> Template {
    // let mut cnxt = tera::Context::new();
    // cnxt.insert("user", &user);
    Template::render("index", json!({ "user": &user }))
}

#[get("/logout")]
fn logout(mut auth: Auth) -> Template {
    auth.logout().unwrap();
    Template::render("logout", json!({}))
}
#[get("/delete")]
async fn delete<'a>(mut auth: Auth<'a>) -> &'static str {
    auth.delete().await.unwrap();
    "Your account was deleted."
}



#[rocket::main]
async fn main() -> Result<()> {
    let mut users = Users::open_postgres("database.db").await?;
    users.open_redis("redis://127.0.0.1/").await?;
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                get_login,
                post_signup,
                get_signup,
                post_login,
                logout,
                delete
            ],
        )
        .manage(users)
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
