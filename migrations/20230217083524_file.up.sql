-- Add up migration script here
create table if not exists files (
   id text primary key,
   disk_name text not null,
   file_name text not null,
   dir BOOLEAN NOT NULL DEFAULT 0,
   hostname text not null,
   timestamp integer not null,

   unique(disk_name, file_name, dir)
);

create index if not exists idx_files_file_name on files(file_name);