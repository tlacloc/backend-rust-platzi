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

    // post a new post to the database
    // let new_post = NewPost {
    //     title: "Using Diesel 2",
    //     slug: "using-diesel2",
    //     body: "This is a blog post using Diesel"
    // };

    // diesel::insert_into(posts)
    //     .values(&new_post)
    //     .execute(&conn)
    //     .expect("Error saving new post");

    // edit a post in the database
    // let post_update = diesel::update(posts.find(3))
    //     .set((title.eq("Using Diesel 3"),slug.eq("using-diesel3"), body.eq("This is a blog post using Diesel")))
    //     .execute(&conn)
    //     .expect("Error updating post");

    // delete a post from the database
    diesel::delete(posts.find(3)).execute(&conn).expect("Error deleting post");

    // delete all posts with the same slug structure from the database
    diesel::delete(posts.filter(slug.like("%-diesel%"))).execute(&conn).expect("Error deleting post");


    // get all posts from the database
    let posts_result = posts.load::<Post>(&conn)
        .expect("Error loading posts")
        .iter()
        .for_each(|post| println!("{} {} {} {}", post.id, post.title, post.slug, post.body));

}
