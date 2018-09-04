table! {
    definitions (id) {
        id -> Integer,
        applicant -> Integer,
        argument -> Integer,
    }
}

table! {
    integers (id) {
        id -> Integer,
        result -> Integer,
    }
}

table! {
    reductions (id) {
        id -> Integer,
        normal_form -> Integer,
    }
}

table! {
    text (id) {
        id -> Integer,
        result -> Text,
    }
}

allow_tables_to_appear_in_same_query!(definitions, integers, reductions, text,);
