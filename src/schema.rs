// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (account_id) {
        account_id -> Int4,
        name -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        stars -> Int4,
        about -> Nullable<Varchar>,
    }
}

diesel::table! {
    meshs (mesh_id) {
        mesh_id -> Int4,
        account_id -> Int4,
        mesh_name -> Varchar,
        mesh_description -> Varchar,
        ply_contents -> Varchar,
        uploaded_at -> Timestamp,
        stars -> Int4,
    }
}

diesel::table! {
    model_to_stage_link (link_id) {
        link_id -> Int4,
        model_id -> Int4,
        stage_id -> Int4,
        account_id -> Int4,
        created_at -> Timestamp,
        link_description -> Nullable<Varchar>,
        position_x -> Float8,
        position_y -> Float8,
        position_z -> Nullable<Float8>,
        rotation_x -> Float8,
        rotation_y -> Float8,
        rotation_z -> Float8,
    }
}

diesel::table! {
    models (model_id) {
        model_id -> Int4,
        account_id -> Int4,
        model_name -> Varchar,
        model_description -> Varchar,
        model_ifc -> Varchar,
        uploaded_at -> Timestamp,
        stars -> Int4,
    }
}

diesel::table! {
    projects (project_id) {
        project_id -> Int4,
        account_id -> Int4,
        project_name -> Varchar,
        project_description -> Nullable<Varchar>,
        created_at -> Timestamp,
        stars -> Int4,
    }
}

diesel::table! {
    stages (stage_id) {
        stage_id -> Int4,
        project_id -> Int4,
        mesh_id -> Int4,
        account_id -> Int4,
        stage_description -> Nullable<Varchar>,
        position_z -> Float8,
        created_at -> Timestamp,
        stars -> Int4,
    }
}

diesel::joinable!(meshs -> accounts (account_id));
diesel::joinable!(model_to_stage_link -> accounts (account_id));
diesel::joinable!(model_to_stage_link -> models (model_id));
diesel::joinable!(model_to_stage_link -> stages (stage_id));
diesel::joinable!(models -> accounts (account_id));
diesel::joinable!(projects -> accounts (account_id));
diesel::joinable!(stages -> accounts (account_id));
diesel::joinable!(stages -> meshs (mesh_id));
diesel::joinable!(stages -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    meshs,
    model_to_stage_link,
    models,
    projects,
    stages,
);
