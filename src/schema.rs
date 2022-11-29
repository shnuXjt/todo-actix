// @generated automatically by Diesel CLI.

diesel::table! {
    todo_item (id) {
        id -> Int4,
        title -> Varchar,
        checked -> Bool,
        list_id -> Int4,
    }
}

diesel::table! {
    todo_list (id) {
        id -> Int4,
        title -> Varchar,
    }
}

diesel::joinable!(todo_item -> todo_list (list_id));

diesel::allow_tables_to_appear_in_same_query!(
    todo_item,
    todo_list,
);
