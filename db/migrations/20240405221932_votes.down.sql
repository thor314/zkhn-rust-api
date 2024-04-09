-- Add down migration script here

drop table if exists user_votes;
drop type if exists item_or_comment_enum;
drop type if exists vote_state_enum;