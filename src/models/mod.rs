use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub mod schema;

// Estruturas para Users
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub cargo: String,
}

// Enum para Status da Tarefa
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StatusTarefa {
    Pendente,
    EmAndamento,
    Concluida,
    Cancelada,
}

// Estruturas para Tarefas
#[derive(Debug, Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = tecnico_id))]
#[diesel(table_name = crate::schema::tarefas)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tarefa {
    pub id: i32,
    pub descricao: Option<String>,
    pub realizado_em: Option<NaiveDateTime>,
    pub tecnico_id: i32,
    pub criado_em: NaiveDateTime,
    pub atualizado_em: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::tarefas)]
pub struct NewTarefa {
    pub descricao: String,
    pub tecnico_id: i32,
}

// Estrutura para atualização de Tarefa
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::tarefas)]
pub struct TarefaUpdate {
    pub descricao: Option<String>,
    pub realizado_em: Option<NaiveDateTime>,
}

// Estrutura para Login
#[derive(Debug, Deserialize)]
pub struct LogInfo {
    pub email: String,
    pub senha: String,
}

// Estrutura para resposta de autenticação
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}
