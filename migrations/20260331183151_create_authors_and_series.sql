CREATE TABLE IF NOT EXISTS authors (
    id BIGSERIAL PRIMARY KEY,
    "name" varchar(512) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_authors_name ON things.authors USING btree (name);

INSERT INTO authors (id, name) VALUES (1, 'Author1');
INSERT INTO authors (id, name) VALUES (2, 'Author2');
INSERT INTO authors (id, name) VALUES (3, 'Author3');
INSERT INTO authors (id, name) VALUES (4, 'Author4');
INSERT INTO authors (id, name) VALUES (5, 'Author5');
-------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS series (
    id BIGSERIAL PRIMARY KEY,
    "name" varchar(512) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_series_name ON things.series USING btree (name);

INSERT INTO series (id, name) values (1, 'series 1');
INSERT INTO series (id, name) values (2, 'series 2');
INSERT INTO series (id, name) values (3, 'series 3');
INSERT INTO series (id, name) values (4, 'series 4');
INSERT INTO series (id, name) values (5, 'series 5');
INSERT INTO series (id, name) values (6, 'series 6');
INSERT INTO series (id, name) values (7, 'series 7');
INSERT INTO series (id, name) values (8, 'series 8');
INSERT INTO series (id, name) values (9, 'series 9');
INSERT INTO series (id, name) values (10, 'series 10');
----------------------------------------------------------------
CREATE TABLE IF NOT EXISTS places (
    id BIGSERIAL PRIMARY KEY,
    name varchar(512) NOT NULL,
    parent_id BIGINT,
    description varchar(512),
    level BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_places_level ON places (level);
CREATE INDEX idx_places_name ON places (name);
CREATE INDEX idx_places_parent ON places (parent_id);
CREATE INDEX idx_places_parent_level ON places (parent_id, level);

INSERT INTO places (id, name, parent_id, description, level) VALUES
 (1, 'Home', NULL, 'квартира на Борщаговке', 1),
 (2, 'Hall', 1, 'коридор на Борщаговке', 2),
 (3, 'Room', 1, 'комната на Борщаговке', 2),
 (4, 'Balcony', 1, 'балкон на Борщаговке', 2),
 (5, 'Shelf', 2, 'шкаф в коридоре', 3),
 (6, 'Shelf', 3, 'шкаф в комнате', 3),
 (7, 'Shelf', 4, 'шкаф на балконе', 3),

 (8, 'Dacha', NULL, 'дача', 1),
 (9, 'Garage', 8, 'гараж на даче', 2),
 (10, 'Basement', 8, 'комната в подвале', 2),
 (11, 'Kitchen', 8, 'кухня на даче', 2),
 (12, 'Living room', 8, 'гостиная на 1 этаже', 2),
 (13, 'Room 1 on 2 floor', 8, 'комната 1 на 2 этаже', 2),
 (14, 'Room 2 on 2 floor', 8, 'комната 2 на 2 этаже', 2),
 (15, 'Hall on 2 floor', 8, 'коридор на 2 этаже', 2),
 (16, 'Attic', 8, 'чердак на даче', 2),
 (17, 'Utility block', 8, 'хозблок', 2),

 (18, 'Shelf', 9, 'стелаж в гараже', 3),
 (19, 'Shelf', 10, 'стелаж в подвале', 3),
 (20, 'Nightstand', 12, 'тумба в гостинной', 3),
 (21, 'Shelf', 12, 'шкаф в гостинной', 3),
 (22, 'Shelf', 13, 'шкаф в 1-й комнате', 3),
 (23, 'Shelf', 14, 'шкаф во 2-й комнате', 3),
 (24, 'Book shelf 1', 15, 'книжный 1 на 2-м этаже', 3),
 (25, 'Book shelf 2', 15, 'книжный 2 на 2-м этаже', 3),
 (26, 'Floor', 16, 'пол на чердаке', 3),
 (27, 'Bathroom', 17, 'ванная', 3),
 (28, 'Sauna', 17, 'баня', 3),
 (29, 'Henhouse', 17, 'курятник', 3),

 (30, 'Mother flat', NULL, 'квартира мамы', 1),
 (31, 'Big room mother flat', 30, 'большая комната маминой квартиры', 2),
 (32, 'Small room mother flat', 30, 'маленькая комната маминой квартиры', 2),
 (33, 'Hall mother flat', 30, 'коридор маминой квартиры', 2),
 (34, 'Shelf', 31, 'шкаф в большой комнате', 3),
 (35, 'Shelf', 32, 'шкаф в маленькой комнате', 3),
 (36, 'Shelf', 33, 'шкаф в коридоре', 3);
----------------------------------------------------------------
CREATE TABLE genre (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL UNIQUE,
  note VARCHAR(512),
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_genre_name ON genre USING btree (name);

INSERT INTO genre (id, name, note) VALUES
  (100, 'ROMANCE', 'Любовный роман'),
  (110, 'FICTION', 'Роман'),
  (120, 'FANTASY', 'Фэнтези'),
  (130, 'MYSTERY', 'Детектив'),
  (140, 'THRILLER', 'Триллер'),
  (150, 'ADVENTURE', 'Приключения'),
  (160, 'HISTORICAL_FICTION', 'Исторический роман'),
  (170, 'BIOGRAPHY', 'Биография'),
  (180, 'MEMOIR', 'Мемуары'),
  (190, 'ACADEMIC', 'Научная литература'),
  (200, 'POPULAR_SCIENCE', 'Научно-популярная литература'),
  (210, 'PHILOSOPHY', 'Философия'),
  (220, 'POETRY', 'Поэзия'),
  (230, 'DRAMA', 'Драма'),
  (240, 'COMEDY', 'Комедия'),
  (250, 'HORROR', 'Ужасы'),
  (260, 'RELIGIOUS', 'Религиозная литература'),
  (270, 'ESSAY', 'Эссе'),
  (280, 'REFERENCE', 'Справочная литература'),
  (290, 'CHILDREN', 'Детская литература'),
  (300, 'HUMOR', 'Юмор и сатира'),
  (310, 'FOLKLORE', 'Фольклор'),
  (320, 'MYTHOLOGY', 'Мифология'),
  (330, 'EPIC', 'Эпопея'),
  (340, 'EROTIC', 'Эротическая литература'),
  (350, 'JOURNALISM', 'Публицистика'),
  (360, 'POLITICAL', 'Политическая литература'),
  (370, 'CYBERPUNK', 'Киберпанк'),
  (380, 'POST_APOCALYPTIC', 'Постапокалипсис'),
  (390, 'ALTERNATE_HISTORY', 'Альтернативная история'),
  (400, 'MAGICAL_REALISM', 'Магический реализм'),
  (410, 'SOCIAL_PROSE', 'Социальная проза'),
  (420, 'CLASSICS', 'Классическая литература'),
  (430, 'CONTEMPORARY', 'Современная литература'),
  (440, 'WAR_FICTION', 'Военная проза'),
  (450, 'LITRPG', 'ЛитРПГ'),
  (460, 'HARD_SCIENCE_FICTION', 'Хай-тек фантастика'),
  (470, 'SPACE_OPERA', 'Космическая опера'),
  (480, 'STEAMPUNK', 'Паропанк'),
  (490, 'DYSTOPIA', 'Антиутопия'),
  (500, 'UTOPIA', 'Утопия'),
  (510, 'GOTHIC', 'Готическая литература'),
  (520, 'POLICE_NOVEL', 'Полицейский роман'),
  (530, 'SPY_NOVEL', 'Шпионский роман'),
  (540, 'PSYCHOLOGICAL_PROSE', 'Психологическая проза'),
  (550, 'CRITICISM', 'Критика и литературоведение'),
  (560, 'PLAY', 'Пьеса'),
  (600, 'IT', 'АйТи'),
  (601, 'ASSEMBLER', 'Ассеблер'),
  (602, 'C++', 'C++'),
  (603, 'C LANGUAGE', 'Язык C'),
  (604, 'ALGORITHMS AND DATA STRUCTURES', 'Алгоритмы и структуры данных'),
  (605, 'NETWORKS', 'Сети'),
  (606, 'CRYPTOGRAPHY', 'Криптография');

----------------------------------------------------------------
CREATE TABLE IF NOT EXISTS books (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    publication_year DATE,
    publisher VARCHAR(255),
    volume_number VARCHAR(50),
    author_id BIGINT REFERENCES authors(id),
    genre_id BIGINT REFERENCES genre(id),
    series_id BIGINT REFERENCES series(id),
    place_id BIGINT REFERENCES places(id),
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
    );
CREATE INDEX idx_books_title ON books USING btree (title);
CREATE INDEX idx_books_genre ON books USING btree (genre_id);
CREATE INDEX idx_books_author ON books USING btree (author_id);
CREATE INDEX idx_books_genre_series ON books USING btree (genre_id, series_id);
CREATE INDEX idx_books_series ON books USING btree (series_id);

INSERT INTO books
(id, publication_year, author_id, title, publisher, volume_number, genre_id, series_id, description, place_id, created_at, updated_at)
VALUES
    (1, '1975-01-01', 1,  'Book 1', 'Publisher 1', '1', 100, 1, 'Description of Book 1', 1, NOW() - INTERVAL '50 years', NOW() - INTERVAL '50 years'),
    (2, '1976-01-01', 2, 'Book 2', 'Publisher 2', '2', 200, 1, 'Description of Book 2', 2, NOW() - INTERVAL '49 years', NOW() - INTERVAL '49 years'),
    (3, '1977-01-01', 3, 'Book 3', 'Publisher 3', '3', 300, 2, 'Description of Book 3', 3, NOW() - INTERVAL '48 years', NOW() - INTERVAL '48 years'),
    (4, '1978-01-01', 4, 'Book 4', 'Publisher 4', '4', 100, 2, 'Description of Book 4', 4, NOW() - INTERVAL '47 years', NOW() - INTERVAL '47 years'),
    (5, '1979-01-01', 5, 'Book 5', 'Publisher 5', '5', 200, 3, 'Description of Book 5', 5, NOW() - INTERVAL '46 years', NOW() - INTERVAL '46 years'),
    (6, '1980-01-01', 1, 'Book 6', 'Publisher 6', '6', 300, 3, 'Description of Book 6', 6, NOW() - INTERVAL '45 years', NOW() - INTERVAL '45 years'),
    (7, '1981-01-01', 2, 'Book 7', 'Publisher 7', '7', 100, 4, 'Description of Book 7', 7, NOW() - INTERVAL '44 years', NOW() - INTERVAL '44 years'),
    (8, '1982-01-01', 3, 'Book 8', 'Publisher 8', '8', 200, 4, 'Description of Book 8', 8, NOW() - INTERVAL '43 years', NOW() - INTERVAL '43 years'),
    (9, '1983-01-01', 4, 'Book 9', 'Publisher 9', '9', 300, 5, 'Description of Book 9', 9, NOW() - INTERVAL '42 years', NOW() - INTERVAL '42 years'),
    (10, '1984-01-01', 5, 'Book 10', 'Publisher 10', '10', 100, 5, 'Description of Book 10', 10, NOW() - INTERVAL '41 years', NOW() - INTERVAL '41 years'),

    (11, '1985-01-01', 1, 'Book 11', 'Publisher 11', '11', 200, 6, 'Description of Book 11', 11, NOW() - INTERVAL '40 years', NOW() - INTERVAL '40 years'),
    (12, '1986-01-01', 2, 'Book 12', 'Publisher 12', '12', 300, 6, 'Description of Book 12', 12, NOW() - INTERVAL '39 years', NOW() - INTERVAL '39 years'),
    (13, '1987-01-01', 3, 'Book 13', 'Publisher 13', '13', 100, 7, 'Description of Book 13', 13, NOW() - INTERVAL '38 years', NOW() - INTERVAL '38 years'),
    (14, '1988-01-01', 4, 'Book 14', 'Publisher 14', '14', 200, 7, 'Description of Book 14', 14, NOW() - INTERVAL '37 years', NOW() - INTERVAL '37 years'),
    (15, '1989-01-01', 5, 'Book 15', 'Publisher 15', '15', 300, 8, 'Description of Book 15', 15, NOW() - INTERVAL '36 years', NOW() - INTERVAL '36 years'),

    (16, '1990-01-01', 1, 'Book 16', 'Publisher 16', '16', 100, 8, 'Description of Book 16', 16, NOW() - INTERVAL '35 years', NOW() - INTERVAL '35 years'),
    (17, '1991-01-01', 2, 'Book 17', 'Publisher 17', '17', 200, 9, 'Description of Book 17', 17, NOW() - INTERVAL '34 years', NOW() - INTERVAL '34 years'),
    (18, '1992-01-01', 3, 'Book 18', 'Publisher 18', '18', 300, 9, 'Description of Book 18', 18, NOW() - INTERVAL '33 years', NOW() - INTERVAL '33 years'),
    (19, '1993-01-01', 4, 'Book 19', 'Publisher 19', '19', 100, 10, 'Description of Book 19', 19, NOW() - INTERVAL '32 years', NOW() - INTERVAL '32 years'),
    (20, '1994-01-01', 5, 'Book 20', 'Publisher 20', '20', 200, 10, 'Description of Book 20', 20, NOW() - INTERVAL '31 years', NOW() - INTERVAL '31 years'),

    (21, '1995-01-01', 1, 'Book 21', 'Publisher 21', '21', 300, 1, 'Description of Book 21', 1, NOW() - INTERVAL '30 years', NOW() - INTERVAL '30 years'),
    (22, '1996-01-01', 2, 'Book 22', 'Publisher 22', '22', 100, 2, 'Description of Book 22', 2, NOW() - INTERVAL '29 years', NOW() - INTERVAL '29 years'),
    (23, '1997-01-01', 3, 'Book 23', 'Publisher 23', '23', 200, 3, 'Description of Book 23', 3, NOW() - INTERVAL '28 years', NOW() - INTERVAL '28 years'),
    (24, '1998-01-01', 4, 'Book 24', 'Publisher 24', '24', 300, 4, 'Description of Book 24', 4, NOW() - INTERVAL '27 years', NOW() - INTERVAL '27 years'),
    (25, '1999-01-01', 5, 'Book 25', 'Publisher 25', '25', 100, 5, 'Description of Book 25', 5, NOW() - INTERVAL '26 years', NOW() - INTERVAL '26 years'),

    (26, '2000-01-01', 1, 'Book 26', 'Publisher 26', '26', 200, 6, 'Description of Book 26', 6, NOW() - INTERVAL '25 years', NOW() - INTERVAL '25 years'),
    (27, '2001-01-01', 2, 'Book 27', 'Publisher 27', '27', 300, 7, 'Description of Book 27', 7, NOW() - INTERVAL '24 years', NOW() - INTERVAL '24 years'),
    (28, '2002-01-01', 3, 'Book 28', 'Publisher 28', '28', 100, 8, 'Description of Book 28', 8, NOW() - INTERVAL '23 years', NOW() - INTERVAL '23 years'),
    (29, '2003-01-01', 4, 'Book 29', 'Publisher 29', '29', 200, 9, 'Description of Book 29', 9, NOW() - INTERVAL '22 years', NOW() - INTERVAL '22 years'),
    (30, '2004-01-01', 5, 'Book 30', 'Publisher 30', '30', 300, 10, 'Description of Book 30', 10, NOW() - INTERVAL '21 years', NOW() - INTERVAL '21 years'),

    (31, '2005-01-01', 1, 'Book 31', 'Publisher 31', '31', 100, 1, 'Description of Book 31', 11, NOW() - INTERVAL '20 years', NOW() - INTERVAL '20 years'),
    (32, '2006-01-01', 2, 'Book 32', 'Publisher 32', '32', 200, 2, 'Description of Book 32', 12, NOW() - INTERVAL '19 years', NOW() - INTERVAL '19 years'),
    (33, '2007-01-01', 3, 'Book 33', 'Publisher 33', '33', 300, 3, 'Description of Book 33', 13, NOW() - INTERVAL '18 years', NOW() - INTERVAL '18 years'),
    (34, '2008-01-01', 4, 'Book 34', 'Publisher 34', '34', 100, 4, 'Description of Book 34', 14, NOW() - INTERVAL '17 years', NOW() - INTERVAL '17 years'),
    (35, '2009-01-01', 5, 'Book 35', 'Publisher 35', '35', 200, 5, 'Description of Book 35', 15, NOW() - INTERVAL '16 years', NOW() - INTERVAL '16 years'),

    (36, '2010-01-01', 1, 'Book 36', 'Publisher 36', '36', 300, 6, 'Description of Book 36', 16, NOW() - INTERVAL '15 years', NOW() - INTERVAL '15 years'),
    (37, '2011-01-01', 2, 'Book 37', 'Publisher 37', '37', 100, 7, 'Description of Book 37', 17, NOW() - INTERVAL '14 years', NOW() - INTERVAL '14 years'),
    (38, '2012-01-01', 3, 'Book 38', 'Publisher 38', '38', 200, 8, 'Description of Book 38', 18, NOW() - INTERVAL '13 years', NOW() - INTERVAL '13 years'),
    (39, '2013-01-01', 4, 'Book 39', 'Publisher 39', '39', 300, 9, 'Description of Book 39', 19, NOW() - INTERVAL '12 years', NOW() - INTERVAL '12 years'),
    (40, '2014-01-01', 5, 'Book 40', 'Publisher 40', '40', 100, 10, 'Description of Book 40', 20, NOW() - INTERVAL '11 years', NOW() - INTERVAL '11 years');
