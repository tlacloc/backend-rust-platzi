#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use tera::Tera;

use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::{NewPost, Post, NewPostHandler};
use self::schema::posts;
use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let conn = pool.get().expect("Failed to get connection from pool");

    match web::block(move || posts.load::<Post>(&conn)).await {
        Ok(data) => {
            let data = data.unwrap();

            let mut context = tera::Context::new();
            context.insert("posts", &data);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("index.html", &context).unwrap()
            )
            
        }
        Err(err) => HttpResponse::Ok().body("Error connecting to Postgres"),
    }
}

#[get("/post/{blog_slug}")]
async fn get_blog(
    pool: web::Data<DbPool>, 
    template_manager: web::Data<tera::Tera>, 
    blog_slug: web::Path<String>
) -> impl Responder {
    
    let conn = pool.get().expect("Failed to get connection from pool");

    let url_slug = blog_slug.into_inner();

    match web::block(move || posts.filter(slug.eq(url_slug)).load::<Post>(&conn)).await {
        Ok(data) => {
            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().body("Post not found");
            }

            let data = &data[0];

            let mut context = tera::Context::new();
            context.insert("post", &data);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("posts.html", &context).unwrap()
            )
            
        }
        Err(err) => HttpResponse::Ok().body("Error connecting to Postgres"),
    }
}


#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, new_post: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Failed to get connection from pool");

    println!("{:?}", new_post);

    match web::block(move || {
        Post::create_post(&conn, &new_post)
    })
    .await
    {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        }
        Err(_) => HttpResponse::Ok().body("Error connecting to Postgres"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let port : u16 = port.parse().unwrap();

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder()
        .build(connection)
        .expect("Failed to create pool.");
        
        // move transfer main to any other thread is needed
        HttpServer::new(move || {
            let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"),"/templates/**/*")).unwrap();

            App::new()
            .service(index)
            .service(new_post)
            .service(get_blog)
            .data(pool.clone())
            .data(tera)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await

    // let conn = PgConnection::establish(&db_url)
    //     .expect(&format!("Error connecting to {}", db_url));

    // use self::models::{Post, NewPost};
    // use self::schema::posts;
    // use self::schema::posts::dsl::*;

    // post a new post to the database
    // let new_post = NewPost {
    //     title: "Using Diesel 3",
    //     slug: "using-diesel3",
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
    // diesel::delete(posts.find(3)).execute(&conn).expect("Error deleting post");

    // delete all posts with the same slug structure from the database
    // diesel::delete(posts.filter(slug.like("%-diesel%"))).execute(&conn).expect("Error deleting post");

    // get all posts from the database
    // let posts_result = posts.load::<Post>(&conn)
    //     .expect("Error loading posts")
    //     .iter()
    //     .for_each(|post| println!("{} {} {} {}", post.id, post.title, post.slug, post.body));
}
