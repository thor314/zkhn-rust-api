-- Add migration script here
DROP TABLE IF EXISTS moderator_action_enum;
CREATE TYPE moderator_action_enum AS ENUM (
  'KillItem',
  'UnkillItem',
  'KillComment',
  'UnkillComment',
  'AddUserShadowBan',
  'RemoveUserShadowBan',
  'AddUserBan',
  'RemoveUserBan'
);

DROP TABLE IF EXISTS moderation_logs;
CREATE TABLE moderation_logs (
    id UUID PRIMARY KEY,
    moderator_username TEXT NOT NULL,
    action_type moderator_action_enum NOT NULL,
    username TEXT,
    item_id UUID,
    item_title TEXT,
    item_by TEXT,
    comment_id UUID,
    comment_by TEXT,
    created TIMESTAMP WITH TIME ZONE NOT NULL
);

