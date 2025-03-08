use chrono::NaiveDate;

// user to course specialization mapping
struct User {
    id: usize,
    first_name: String,
    last_name: String,
    birth_date: NaiveDate
}

struct Course {
    id: usize,
    name: String,
    abbreviation: String,
    start_year: usize,
}

// course specialization to course mapping (n:m)
struct CourseSpecialization {
    id: usize,
    fach_abi: bool,
    shortening: Option<usize>
}

enum ReportKind {
    School,
    Training,
}

enum UserSortField {
    FirstName,
    LastName,
    BirthDate,
}

enum SortDirection {
    Ascending,
    Descending,
}

struct Rotation {
    id: usize,
    course_id: usize,
    course_specialization_id: usize,
    user_sort_field: UserSortField,
    sort_direction: SortDirection,
    report_kind: ReportKind,
}
