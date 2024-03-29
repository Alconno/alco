pub const CODE: &str = r#"// @generated automatically by Diesel CLI.
/*

EXAMPLE USE: 

diesel::table! {
    armory_names (id) {
        #[max_length = 36]
        id -> Varchar,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    name_logs (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        armory_name_id -> Varchar,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(name_logs -> armory_names(armory_name_id));

diesel::allow_tables_to_appear_in_same_query!(
    armory_names,
    name_logs,
);

*/

"#;