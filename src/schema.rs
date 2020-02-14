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
    }
}
