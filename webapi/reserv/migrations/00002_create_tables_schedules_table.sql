create table if not exists tables_schedules (
	schedule_id serial primary key,
	table_id int,
	table_capacity smallint not null,
    reservation_time timestamp not null,
    is_reserved boolean default false,
    unique (table_id, reservation_time)
);