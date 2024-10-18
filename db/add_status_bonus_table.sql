CREATE TABLE ragnarok.status_bonus (
    account_id integer not null,
    char_id integer not null,
    remaining_ms integer not null,
    bonus_type text not null,
    flag integer not null,
    source text,
    source_val1 integer,
    bonus_val1 integer,
    bonus_val2 integer
);

create index status_bonus_index on ragnarok.status_bonus (char_id);