use std::ops::Deref;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

use crate::model::{NewManga, Manga, Genre, Category};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn get_all_mangas(pool: &PgPool) -> Result<Vec<Manga>, &'static str> {
    Manga::all(get_conn(pool)?.deref()).map_err(|_| "Error inserting manga")
}

pub fn create_manga(manga_description: String, manga_genre: String, pool: &PgPool) -> Result<(), &'static str> {
    let new_manga = NewManga { descriptions: manga_description, genre: "default".to_string() };
    Manga::insert(new_manga, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error inserting manga")
}

pub fn create_category( manga_genre: String, pool: &PgPool) -> Result<(), &'static str> {
    let new_genre = Genre { genre:"test".to_string()};
    Category::insert(new_genre, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error inserting manga")
}


pub fn toggle_manga(id: i32, pool: &PgPool) -> Result<(), &'static str> {
    Manga::toggle_with_id(id, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error toggling manga")
}

pub fn delete_manga(id: i32, pool: &PgPool) -> Result<(), &'static str> {
    Manga::delete_with_id(id, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error deleting manga")
}
