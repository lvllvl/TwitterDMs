pub mod auth {
    pub const REQUEST_TOKEN: &str = "https://api.twitter.com/oauth/request_token";
    pub const ACCESS_TOKEN: &str = "https://api.twitter.com/oauth/access_token";
    pub const BEARER_TOKEN: &str = "https://api.twitter.com/oauth2/token";
    pub const INVALIDATE_BEARER: &str = "https://api.twitter.com/oauth2/invalidate_token";
    pub const AUTHORIZE: &str = "https://api.twitter.com/oauth/authorize";
    pub const AUTHENTICATE: &str = "https://api.twitter.com/oauth/authenticate";
    pub const VERIFY_CREDENTIALS: &str =
        "https://api.twitter.com/1.1/account/verify_credentials.json";
}

pub mod users {
    pub const LOOKUP: &str = "https://api.twitter.com/1.1/users/lookup.json";
    pub const SHOW: &str = "https://api.twitter.com/1.1/users/show.json";
    pub const SEARCH: &str = "https://api.twitter.com/1.1/users/search.json";
    pub const FRIENDS_LIST: &str = "https://api.twitter.com/1.1/friends/list.json";
    pub const FRIENDS_IDS: &str = "https://api.twitter.com/1.1/friends/ids.json";
    pub const FOLLOWERS_LIST: &str = "https://api.twitter.com/1.1/followers/list.json";
    pub const FOLLOWERS_IDS: &str = "https://api.twitter.com/1.1/followers/ids.json";
    pub const BLOCKS_LIST: &str = "https://api.twitter.com/1.1/blocks/list.json";
    pub const BLOCKS_IDS: &str = "https://api.twitter.com/1.1/blocks/ids.json";
    pub const MUTES_LIST: &str = "https://api.twitter.com/1.1/mutes/users/list.json";
    pub const MUTES_IDS: &str = "https://api.twitter.com/1.1/mutes/users/ids.json";
    pub const FOLLOW: &str = "https://api.twitter.com/1.1/friendships/create.json";
    pub const UNFOLLOW: &str = "https://api.twitter.com/1.1/friendships/destroy.json";
    pub const FRIENDSHIPS_INCOMING: &str = "https://api.twitter.com/1.1/friendships/incoming.json";
    pub const FRIENDSHIPS_OUTGOING: &str = "https://api.twitter.com/1.1/friendships/outgoing.json";
    pub const FRIENDSHIP_SHOW: &str = "https://api.twitter.com/1.1/friendships/show.json";
    pub const FRIENDSHIP_UPDATE: &str = "https://api.twitter.com/1.1/friendships/update.json";
    pub const FRIENDS_NO_RETWEETS: &str =
        "https://api.twitter.com/1.1/friendships/no_retweets/ids.json";
    pub const FRIENDSHIP_LOOKUP: &str = "https://api.twitter.com/1.1/friendships/lookup.json";
    pub const BLOCK: &str = "https://api.twitter.com/1.1/blocks/create.json";
    pub const UNBLOCK: &str = "https://api.twitter.com/1.1/blocks/destroy.json";
    pub const REPORT_SPAM: &str = "https://api.twitter.com/1.1/users/report_spam.json";
    pub const MUTE: &str = "https://api.twitter.com/1.1/mutes/users/create.json";
    pub const UNMUTE: &str = "https://api.twitter.com/1.1/mutes/users/destroy.json";
}

pub mod direct {
    pub const SHOW: &str = "https://api.twitter.com/1.1/direct_messages/events/show.json";
    pub const LIST: &str = "https://api.twitter.com/1.1/direct_messages/events/list.json";
    pub const SEND: &str = "https://api.twitter.com/1.1/direct_messages/events/new.json";
    pub const DELETE: &str = "https://api.twitter.com/1.1/direct_messages/events/destroy.json";
    pub const MARK_READ: &str = "https://api.twitter.com/1.1/direct_messages/mark_read.json";
    pub const INDICATE_TYPING: &str =
        "https://api.twitter.com/1.1/direct_messages/indicate_typing.json";
}