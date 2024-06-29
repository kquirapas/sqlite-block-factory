-- Add up migration script here
create table block_data (
	id varchar(48) not null,
	hash varchar(32) not null unique,
	nonce bigint not null,
	height bigint not null unique,
	prev_block_hash varchar(32) not null unique,
	primary key (id)
);

create table transaction_data (
	id varchar(48) not null,
	hash varchar(32) not null unique,
	from_address varchar(32) not null,
	to_address varchar(32) not null,
	instruction varchar(32) not null, 
	primary key (id)
);
