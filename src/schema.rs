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
