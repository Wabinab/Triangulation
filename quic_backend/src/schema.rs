// @generated automatically by Diesel CLI.

diesel::table! {
    Localize (locale_id) {
        locale_id -> Integer,
        table_name -> Text,
        foreign_id -> Integer,
        locale -> Text,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    RemQuestion (id) {
        id -> Integer,
        t -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        r_id -> Integer,
    }
}

diesel::table! {
    Reminder (id) {
        id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        s_id -> Integer,
    }
}

diesel::table! {
    Stage (s_id) {
        s_id -> Integer,
        step -> Integer,
        t_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    Template (t_id) {
        t_id -> Integer,
        uuid -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(RemQuestion -> Reminder (r_id));
diesel::joinable!(Reminder -> Stage (s_id));
diesel::joinable!(Stage -> Template (t_id));

diesel::allow_tables_to_appear_in_same_query!(
    Localize,
    RemQuestion,
    Reminder,
    Stage,
    Template,
);
