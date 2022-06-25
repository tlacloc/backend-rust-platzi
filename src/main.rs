#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn = PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url));

    use self::models::{Post, NewPost};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let new_post = NewPost {
        title: "Using Diesel 2",
        slug: "using-diesel2",
        body: "This is a blog post using Diesel"
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(&conn)
        .expect("Error saving new post");

    // Select all posts
    let posts_result = posts.load::<Post>(&conn)
        .expect("Error loading posts")
        .iter()
        .for_each(|post| println!("{}", post.title));

}
