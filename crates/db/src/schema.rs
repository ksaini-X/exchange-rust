diesel::table! {
    users(id){
        id->Uuid,
        username -> VarChar,
        email ->VarChar,
        hashed_password ->VarChar,
    }
}
