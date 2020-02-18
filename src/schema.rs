table! {
    term (date) {
        date -> Integer,
        description -> Text,
    }
}

table! {
    instructor (instructorid) {
        instructorid -> Integer,
        fullname -> Text,
        firstname -> Nullable<Text>,
        lastname -> Nullable<Text>,
        rating -> Nullable<Double>,
        url -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
    }
}

table! {
    course (courseid) {
        courseid -> Integer,
        name -> Text,
        class -> Text,
        discipline -> Text,
    }
}

table! {
    classoffering (classofferingid) {
        classofferingid -> Integer,
        courseid -> Integer,
        instructorid -> Nullable<Integer>,
        term -> Nullable<Integer>,
        credits -> Integer,
        days -> Nullable<Text>,
        time -> Nullable<Text>,
        crn -> Integer,
        timestamp -> Nullable<Timestamp>,
    }
}

joinable!(classoffering -> course (courseid));
joinable!(classoffering -> instructor (instructorid));
joinable!(classoffering -> term (term));

allow_tables_to_appear_in_same_query!(classoffering, course, instructor, term,);
