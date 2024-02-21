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
        children_count -> Int4,
        points -> Int4,
        created -> Timestamptz,
        dead -> Bool,
    }
}
