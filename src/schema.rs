// @generated automatically by Diesel CLI.

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
        children_count -> Nullable<Int4>,
        points -> Nullable<Int4>,
        created -> Timestamptz,
        dead -> Nullable<Bool>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(comments, posts,);
