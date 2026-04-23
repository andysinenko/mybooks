SELECT setval(
               pg_get_serial_sequence('books', 'id'),
               (SELECT MAX(id) FROM books)); -- shitty behavior

select * from genre g ;

select * from series s ;

select * from places p ;

select * from books b;