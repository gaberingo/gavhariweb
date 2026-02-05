// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Int8,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 64]
        icon -> Nullable<Varchar>,
        #[max_length = 64]
        badge -> Nullable<Varchar>,
        tags -> Array<Nullable<Text>>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
