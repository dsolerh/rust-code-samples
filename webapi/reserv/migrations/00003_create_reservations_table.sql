create table if not exists reservations (
	reservation_id serial primary key,
	reservation_time timestamp not null,
	user_id uuid not null,
	schedule_id int,
	details varchar(200),
	party_size smallint not null,
	unique (schedule_id),
	unique (user_id, reservation_time),
	constraint fk_schedules
    	FOREIGN KEY(schedule_id)
    	references tables_schedules(schedule_id)
    	on delete set null
);