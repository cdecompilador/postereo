-- Migrations usually are numbered with unix time, but simple numbers work too,
-- its good practice to number the initial MVP with just numbers and later on
-- use unix time for iterations

create table if not exists post (
    post_id     integer primary key autoincrement,
    sender      text not null,
    post_time   timestamp not null default current_timestamp,
    message     text not null
);
