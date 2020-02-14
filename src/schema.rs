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
        firstname -> Text,
        lastname -> Text,
        rating -> Double,
        url -> Text,
    }
}
