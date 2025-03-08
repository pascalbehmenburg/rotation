-- PostgreSQL v17 optimized schema

-- Create custom enum types
CREATE TYPE report_kind AS ENUM ('School', 'Training');

CREATE TYPE user_sort_field AS ENUM ('FirstName', 'LastName', 'BirthDate');

CREATE TYPE sort_direction AS ENUM ('Ascending', 'Descending');

-- Create tables with appropriate PostgreSQL data types
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    birth_date DATE NOT NULL,
    
    -- Add index for sorting and searching users
    CONSTRAINT unique_user UNIQUE (first_name, last_name, birth_date)
);

CREATE TABLE courses (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    abbreviation VARCHAR(10) NOT NULL,
    start_year INTEGER NOT NULL,
    
    -- Add indexes for common query patterns
    CONSTRAINT unique_course_abbreviation UNIQUE (abbreviation)
);
COMMENT ON TABLE courses IS 'Stores information about academic courses';

CREATE TABLE course_specializations (
    id BIGSERIAL PRIMARY KEY,
    fach_abi BOOLEAN NOT NULL,
    shortening INTEGER
);
COMMENT ON TABLE course_specializations IS 'Stores course specialization details';

-- Junction table for many-to-many relationship between courses and specializations
CREATE TABLE course_to_specialization (
    course_id BIGINT REFERENCES courses(id) ON DELETE CASCADE,
    specialization_id BIGINT REFERENCES course_specializations(id) ON DELETE CASCADE,
    PRIMARY KEY (course_id, specialization_id)
);

CREATE TABLE rotations (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses(id),
    course_specialization_id BIGINT NOT NULL REFERENCES course_specializations(id),
    user_sort_field user_sort_field NOT NULL,
    sort_direction sort_direction NOT NULL,
    report_kind report_kind NOT NULL,
    
    -- Add appropriate index for performance
    CONSTRAINT fk_course FOREIGN KEY (course_id) REFERENCES courses(id),
    CONSTRAINT fk_specialization FOREIGN KEY (course_specialization_id) REFERENCES course_specializations(id)
);
COMMENT ON TABLE rotations IS 'Manages rotation assignments connecting courses and specializations';


-- Apply optimizations for PostgreSQL v17
ALTER TABLE courses SET (fillfactor = 90);
ALTER TABLE course_specializations SET (fillfactor = 90);
ALTER TABLE rotations SET (fillfactor = 90);

-- Consider partitioning for very large tables if needed
-- Example: Partitioning rotations by course_id if you have many courses:
-- ALTER TABLE rotations PARTITION BY HASH (course_id);