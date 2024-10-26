// @generated automatically by Diesel CLI.

diesel::table! {
    tarefas (id) {
        id -> Int4,
        #[max_length = 2500]
        descricao -> Nullable<Varchar>,
        realizado_em -> Nullable<Timestamp>,
        tecnico_id -> Int4,
        criado_em -> Timestamp,
        atualizado_em -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        nome -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        senha_hash -> Varchar,
        #[max_length = 20]
        cargo -> Varchar,
        criado_em -> Timestamp,
        atualizado_em -> Timestamp,
    }
}

diesel::joinable!(tarefas -> users (tecnico_id));

diesel::allow_tables_to_appear_in_same_query!(
    tarefas,
    users,
);
