use serde::{Deserialize, Serialize};


#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String
}

use super::schema::posts;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub body: &'a str
}

impl Post {
    pub fn create_slug(title: &str) -> String {
        let mut slug = title.to_lowercase();
        slug = slug.replace(" ", "-");
        slug = slug.replace(".", "");
        slug = slug.replace("'", "");
        slug = slug.replace("&", "");
        slug = slug.replace("!", "");
        slug = slug.replace("?", "");
        slug = slug.replace("(", "");
        slug = slug.replace(")", "");
        slug = slug.replace("[", "");
        slug = slug.replace("]", "");
        slug = slug.replace("{", "");
        slug = slug.replace("}", "");
        slug = slug.replace("/", "");
        slug = slug.replace("\\", "");
        slug = slug.replace("*", "");
        slug = slug.replace("%", "");
        slug = slug.replace("$", "");
        slug = slug.replace("#", "");
        slug = slug.replace("@", "");
        slug = slug.replace("+", "");
        slug = slug.replace("=", "");
        slug = slug.replace("|", "");
        slug = slug.replace("~", "");
        slug = slug.replace("`", "");
        slug = slug.replace("^", "");
        slug = slug.replace("\"", "");
        slug = slug.replace("'", "");
        slug = slug.replace(";", "");
        slug = slug.replace(",", "");
        slug = slug.replace("<", "");
        slug = slug.replace(">", "");
        slug = slug.replace(".", "");
        slug = slug.replace("/", "");
        slug = slug.replace("\\", "");
        slug = slug.replace("*", "");
        slug = slug.replace("%", "");
        slug = slug.replace("$", "");
        slug = slug.replace("#", "");
        slug = slug.replace("@", "");
        slug = slug.replace("+", "");
        slug
    }

    pub fn create_post<'a>(conn: &PgConnection, post: &NewPostHandler) -> Result<Post, diesel::result::Error> {
        let slug = Post::create_slug(&post.title.clone());

        let new_post = NewPost {
            title: &post.title,
            slug: &slug,
            body: &post.body
        };

        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result::<Post>(conn)
        
    }

    // pub fn new(title: &str, slug: &str, body: &str) -> NewPost {
    //     NewPost {
    //         title: title,
    //         slug: slug,
    //         body: body
    //     }
    // }
}