table! {
    users (id) {
        id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        user_name -> Varchar,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        pin -> Varchar,
    }
}
