table! {
    fragments (id) {
        id -> Int4,
        image_url -> Text,
        comment -> Text,
        post_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        sub_title -> Text,
    }
}

joinable!(fragments -> posts (post_id));

allow_tables_to_appear_in_same_query!(
    fragments,
    posts,
);
