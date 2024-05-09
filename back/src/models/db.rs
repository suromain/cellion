use chrono::NaiveDateTime;
use diesel::{dsl::Eq, prelude::*};

use crate::schema;

pub type InsertSolution<'a> = (
    Eq<schema::solutions::filename, &'a str>,
    Eq<schema::solutions::created_at, &'a NaiveDateTime>,
);

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::solutions)]
pub struct Solution {
    pub id: i32,
    pub filename: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = crate::schema::teachers)]
#[diesel(belongs_to(Solution))]
pub struct Teacher {
    pub name: String,
    pub solution_id: i32,
    pub department: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(belongs_to(Solution))]
#[diesel(table_name = crate::schema::rooms)]
pub struct Room {
    pub id: String,
    pub solution_id: i32,
    pub capacity: i32,
    pub name: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(belongs_to(Solution))]
#[diesel(table_name = crate::schema::courses)]
pub struct Course {
    pub id: String,
    pub solution_id: i32,
    pub name: Option<String>,
}

#[derive(Hash, Eq, PartialEq, Queryable, Selectable, Insertable, Debug)]
#[diesel(belongs_to(Course))]
#[diesel(table_name = crate::schema::parts)]
pub struct Part {
    pub solution_id: i32,
    pub id: String,
    pub course_id: String,

    pub session_teachers: Option<i32>,
    pub session_rooms: Option<String>,
    pub label: Option<String>,
    pub max_head_count: Option<i32>,
    pub nr_session: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(belongs_to(Part))]
#[diesel(table_name = crate::schema::classes)]
pub struct Class {
    pub solution_id: i32,
    pub id: String,
    pub part_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(belongs_to(Part))]
#[diesel(table_name = crate::schema::students )]
pub struct Student {
    pub solution_id: i32,
    pub id: String,
    pub label: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = crate::schema::students_groups)]
pub struct StudentGroupOwn {
    pub solution_id: i32,
    pub student_id: String,
    pub group_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = crate::schema::sessions)]
pub struct Session {
    pub solution_id: i32,
    pub uuid: String,
    pub class_id: String,
    pub rank: i32,
    pub starting_date: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = crate::schema::sessions_rooms)]
pub struct SessionRoomOwn {
    pub solution_id: i32,
    pub class_id: String,
    pub session_rank: i32,
    pub room_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = crate::schema::sessions_teachers)]
pub struct SessionTeacherOwn {
    pub solution_id: i32,
    pub class_id: String,
    pub session_rank: i32,
    pub teacher_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = crate::schema::classes_groups)]
pub struct ClassGroupOwn {
    pub solution_id: i32,
    pub class_id: String,
    pub group_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = crate::schema::classes_teachers)]
pub struct ClassTeacherOwn {
    pub solution_id: i32,
    pub class_id: String,
    pub teacher_id: String,
}

#[derive(Queryable, Selectable, Insertable, Debug, Hash, Eq, PartialEq)]
#[diesel(table_name = crate::schema::classes_rooms)]
pub struct ClassRoomOwn {
    pub solution_id: i32,
    pub class_id: String,
    pub room_id: String,
}

sql_function! {
    fn last_insert_rowid() -> Integer
}
