-- Your SQL goes here


create table if not exists users (
    id serial primary key,
    nome varchar(100) not null,
    email varchar(100) not null unique,
    senha_hash varchar not null,
    cargo varchar(20) not null,
    criado_em TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create table if not exists tarefas(
    id serial primary key,
    descricao varchar(2500),
    realizado_em timestamp,
    tecnico_id integer not null references users(id),
    criado_em TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
