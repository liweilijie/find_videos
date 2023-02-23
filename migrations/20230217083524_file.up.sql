-- Add up migration script here
create table if not exists file (
   id text primary key,
   full_path text not null,
   file_name text not null,
   hostname text not null,
   dir boolean not null default 0,
   timestamp integer not null,

   unique(full_path)
);

create index if not exists idx_file_file_name on file(file_name);
