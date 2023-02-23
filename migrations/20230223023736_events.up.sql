-- Add up migration script here
create table if not exists events (
                                      id text primary key,
                                      timestamp integer not null,
                                      hostname text not null,
                                      event_type text not null,
                                      full_path text not null
);

-- Ensure there is only ever one of each event type per file item
create unique index if not exists file_event_idx ON events(event_type, full_path);