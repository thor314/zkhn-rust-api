// @generated automatically by Diesel CLI.

pub mod sql_types {
  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "item_category_enum"))]
  pub struct ItemCategoryEnum;

  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "item_type"))]
  pub struct ItemType;

  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "moderator_action_enum"))]
  pub struct ModeratorActionEnum;

  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "user_vote_type"))]
  pub struct UserVoteType;
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
    use super::sql_types::ItemType;
    use super::sql_types::ItemCategoryEnum;

    items (id) {
        id -> Uuid,
        by -> Text,
        title -> Text,
        item_type -> ItemType,
        url -> Nullable<Text>,
        domain -> Nullable<Text>,
        text -> Nullable<Text>,
        points -> Int4,
        score -> Int4,
        comment_count -> Int4,
        item_category -> ItemCategoryEnum,
        created -> Timestamptz,
        dead -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ModeratorActionEnum;

    moderation_logs (id) {
        id -> Uuid,
        moderator_username -> Text,
        action_type -> ModeratorActionEnum,
        username -> Nullable<Text>,
        item_id -> Nullable<Uuid>,
        item_title -> Nullable<Text>,
        item_by -> Nullable<Text>,
        comment_id -> Nullable<Uuid>,
        comment_by -> Nullable<Text>,
        created -> Timestamptz,
    }
}

diesel::table! {
    user_favorites (username, item_type, item_id) {
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 50]
        item_type -> Varchar,
        item_id -> Uuid,
        date -> Timestamp,
    }
}

diesel::table! {
    user_hiddens (username, item_id) {
        #[max_length = 255]
        username -> Varchar,
        item_id -> Uuid,
        date -> Timestamp,
        item_creation_date -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserVoteType;

    user_votes (username, content_id, vote_type) {
        #[max_length = 255]
        username -> Varchar,
        vote_type -> UserVoteType,
        content_id -> Uuid,
        parent_item_id -> Nullable<Uuid>,
        upvote -> Bool,
        downvote -> Bool,
        date -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
        auth_token -> Nullable<Text>,
        auth_token_expiration -> Nullable<Int8>,
        reset_password_token -> Nullable<Text>,
        reset_password_token_expiration -> Nullable<Int8>,
        email -> Text,
        created -> Timestamptz,
        karma -> Int4,
        about -> Nullable<Text>,
        show_dead -> Bool,
        is_moderator -> Bool,
        shadow_banned -> Bool,
        banned -> Bool,
    }
}

diesel::joinable!(user_favorites -> items (item_id));
diesel::joinable!(user_hiddens -> items (item_id));
diesel::joinable!(user_votes -> comments (content_id));
diesel::joinable!(user_votes -> items (content_id));

diesel::allow_tables_to_appear_in_same_query!(
  comments,
  items,
  moderation_logs,
  user_favorites,
  user_hiddens,
  user_votes,
  users,
);
