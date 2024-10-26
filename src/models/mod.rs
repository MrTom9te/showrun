use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub mod schema;

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub cargo: String,
    pub criado_em: NaiveDateTime,
    pub atualizado_em: NaiveDateTime,
}

#[derive(Debug, Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = tecnico_id))]
#[diesel(table_name = crate::schema::tarefas)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tarefa {
    pub id: i32,
    pub descricao: Option<String>,           // nullable field
    pub realizado_em: Option<NaiveDateTime>, // nullable field
    pub tecnico_id: i32,                     // referencia para o User
    pub criado_em: NaiveDateTime,
    pub atualizado_em: NaiveDateTime,
}
