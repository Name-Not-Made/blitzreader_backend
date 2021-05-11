use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::{
    mangas,
    mangas::dsl::{completed as manga_completed, mangas as all_mangas},
    categories,
    categories::dsl::{categories as category},
};

#[derive(Debug, Insertable)]
#[table_name = "mangas"]
pub struct NewManga {
    pub descriptions: String,
    pub genre: String,

}
#[derive(Debug, Insertable)]
#[table_name = "categories"]
pub struct Genre {
    pub genre: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Manga {
    pub id: i32,
    pub descriptions: String,
    pub genre: String,
    pub reading_status: String,
    pub completed: bool,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Category {
    pub c_id: i32,
    pub genre: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl Manga {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Manga>> {
        all_mangas.order(mangas::id.desc()).load::<Manga>(conn)
    }

    pub fn insert(new_manga: NewManga, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(mangas::table)
            .values(&new_manga)
            .execute(conn)
    }

    pub fn toggle_with_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        let manga_list = all_mangas.find(id).get_result::<Manga>(conn)?;

        let new_manga = !manga_list.completed;
        let updated_list = diesel::update(all_mangas.find(id));
        updated_list
            .set(manga_completed.eq(new_manga))
            .execute(conn)
    }

    pub fn delete_with_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(all_mangas.find(id)).execute(conn)
    }
}

impl Category {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Category>> {
        category.order(categories::c_id.desc()).load::<Category>(conn)
    }

    pub fn insert(genre: Genre, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(categories::table)
            .values(&genre)
            .execute(conn)
    }

    
}


