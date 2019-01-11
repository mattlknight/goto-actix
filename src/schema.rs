table! {
    keywords (row_id) {
        row_id -> Int4,
        keyword -> Text,
        url -> Text,
        created_on -> Timestamp,
        modified_on -> Timestamp,
    }
}

table! {
    tracking (row_id) {
        row_id -> Int4,
        keyword_id -> Int4,
        accessed_on -> Timestamp,
    }
}

joinable!(tracking -> keywords (keyword_id));

allow_tables_to_appear_in_same_query!(
    keywords,
    tracking,
);
