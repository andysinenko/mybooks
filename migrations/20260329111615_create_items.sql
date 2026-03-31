-- Add migration script here
CREATE TABLE books (
                              id int8 NOT NULL,
                              publication_year date NULL,
                              title varchar(255) NULL,
                              publisher varchar(512) NULL,
                              volume_number varchar(512) NULL,
                              genre_id int8 NULL,
                              series_id int8 NULL,
                              description varchar(512) NULL,
                              place_id int8 NULL,
                              created_at TIMESTAMPTZ DEFAULT NOW(),
                              updated_at TIMESTAMPTZ DEFAULT NOW()
                              CONSTRAINT pk_books PRIMARY KEY (id)
);
CREATE INDEX idx_books_genre ON books USING btree (genre_id);
CREATE INDEX idx_books_genre_series ON books USING btree (genre_id, series_id);
CREATE INDEX idx_books_place ON books USING btree (place_id);
CREATE INDEX idx_books_publisher ON books USING btree (publisher);
CREATE INDEX idx_books_series ON books USING btree (series_id);
CREATE INDEX idx_books_title ON books USING btree (title);

