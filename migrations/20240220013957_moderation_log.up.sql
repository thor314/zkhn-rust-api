-- Add up migration script here
CREATE TYPE moderator_action_type AS ENUM (
  'KillItem',
  'UnkillItem',
  'KillComment',
  'UnkillComment',
  'AddUserShadowBan',
  'RemoveUserShadowBan',
  'AddUserBan',
  'RemoveUserBan',
);

CREATE TABLE moderation_logs (
    id UUID PRIMARY KEY,
    moderator_username TEXT NOT NULL,
    -- action_type TEXT NOT NULL CHECK (action_type IN ('kill-item', 'unkill-item', 'kill-comment', 'unkill-comment', 'add-user-shadow-ban', 'remove-user-shadow-ban', 'add-user-ban', 'remove-user-ban')),
    action_type moderator_action_type NOT NULL,
    username TEXT,
    item_id UUID,
    item_title TEXT,
    item_by TEXT,
    comment_id UUID,
    comment_by TEXT,
    created TIMESTAMP WITH TIME ZONE NOT NULL
);