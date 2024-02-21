// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "category_enum"))]
    pub struct CategoryEnum;
}

diesel::table! {
    comments (id) {
        id -> Uuid,
        by -> Text,
        parent_item_id -> Uuid,
        parent_item_title -> Text,
        text -> Text,
        is_parent -> Bool,
        root_comment_id -> Uuid,
        parent_comment_id -> Nullable<Uuid>,
        children_count -> Int4,
        points -> Int4,
        created -> Timestamptz,
        dead -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CategoryEnum;

    items (id) {
        id -> Uuid,
        by -> Text,
        title -> Text,
        item_type -> Text,
        url -> Nullable<Text>,
        domain -> Nullable<Text>,
        text -> Nullable<Text>,
        points -> Int4,
        score -> Int4,
        comment_count -> Int4,
        category -> CategoryEnum,
        created -> Timestamptz,
        dead -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    items,
);
