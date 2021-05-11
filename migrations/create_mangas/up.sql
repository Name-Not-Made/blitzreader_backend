CREATE TABLE IF NOT EXISTS mangas (
  id SERIAL PRIMARY KEY,
  descriptions VARCHAR NOT NULL,
  genre VARCHAR NOT NULL,
  reading_status VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT 'f',
  
);
