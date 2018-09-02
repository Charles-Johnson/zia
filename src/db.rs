use diesel::prelude::*;
pub use diesel::result::{QueryResult, ConnectionError, Error};
pub use diesel::sqlite::SqliteConnection;
use schema::{definitions, text, reductions, integers};
use diesel::{insert_into, update, delete};
use std::fmt;

#[cfg(test)]
mod tests {
    use super::{LUID, DEFINE};
    use super::select_integer;
    use super::memory_database;
    use super::id_from_label;
    #[test]
    fn luid() {
        let conn = memory_database().unwrap();
        let luid = select_integer(LUID, &conn).unwrap();
        assert_eq!(luid, LUID+10);
    }
    #[test]
    fn luid_label() {
        let conn = memory_database().unwrap();
        let luid_id = id_from_label("luid", &conn).unwrap();
        match luid_id {   None => panic!("Could not find label!"),
                       Some(a) => assert_eq!(a, LUID)};
    }
    #[test]
    fn define_label() {
        let conn = memory_database().unwrap();
        let define_id = id_from_label(":=", &conn).unwrap();
        match define_id {   None => panic!("Could not find label!"),
                         Some(a) => assert_eq!(a, DEFINE)};
    }
}

pub const LUID: i32 = -2147483648;
const LABEL: i32 = LUID+1;
pub const DEFINE: i32 = LUID+2;
pub const REDUCTION: i32 = LUID+3;

#[derive(Debug)]
pub enum DBError {
    Query(Error),
    Conn(ConnectionError),
    ZiaConn(String),
    Ambiguity(String),
    Redundancy(String),
    Absence(String),
    Syntax(String)
}

pub type ZiaResult<T> = Result<T, DBError>;

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {DBError::Query(e) => e.fmt(f),
                     DBError::Conn(e) => e.fmt(f),
                    DBError::ZiaConn(s) => write!(f, "{}", s),
                    DBError::Ambiguity(s) => write!(f, "{}", s),
                    DBError::Redundancy(s) => write!(f, "{}", s),
                    DBError::Absence(s) => write!(f, "{}", s),
                     DBError::Syntax(s) => write!(f, "{}", s)
                    }
    }
}

impl From<Error> for DBError {
    fn from(error: Error) -> Self {
        DBError::Query(error)
    }
}

pub fn memory_database() -> ZiaResult<SqliteConnection> {
    let conn = try!(establish_connection(":memory:"));
    try!(setup_database(&conn));
    Ok(conn)
}

fn establish_connection(database_name: &str) -> ZiaResult<SqliteConnection> {
    match SqliteConnection::establish(&database_name) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(DBError::Conn(e))
    }
}

fn setup_database(conn: &SqliteConnection) -> ZiaResult<()>{
    try!(setup_tables(conn));
    try!(insert_integer(LUID, LUID, conn));
    try!(assign_new_id(conn)); //LUID occupies an id
    try!(assign_new_id(conn)); //LABEL occupies an id
    try!(assign_new_id(conn)); //DEFINE occupies an id
    try!(assign_new_id(conn)); //REDUCTION occupies an id
    try!(label_id(LUID, "luid", conn)); //two more ids occupied
    try!(label_id(DEFINE, ":=", conn)); //two more ids occupied
    try!(label_id(REDUCTION, "->", conn)); //two more ids occupied
    Ok(())
}

fn setup_tables(conn: &SqliteConnection) -> QueryResult<()> {
    let string: String = "create table definitions (id integer PRIMARY KEY, applicant integer NOT NULL, argument integer NOT NULL); create table text (id integer PRIMARY KEY, result text NOT NULL); create table reductions (id integer PRIMARY KEY, normal_form integer NOT NULL); create table integers (id integer PRIMARY KEY, result integer NOT NULL);".to_string();
    try!(conn.execute(&string));
    Ok(())
}

// Functions that operate over multiple tables.

pub fn label_id(id: i32, label: &str, conn: &SqliteConnection) -> ZiaResult<()> {
    let definition_id = try!(insert_definition(LABEL, id, conn));
    let text_id = try!(insert_reduction(definition_id, conn));
    try!(insert_text(text_id, label, conn));
    Ok(())
}

fn insert_reduction(id: i32, conn: &SqliteConnection) -> ZiaResult<i32>{
    let normal_form_id = try!(assign_new_id(conn));
    try!(insert_reduction3(id, normal_form_id, conn));
    Ok(normal_form_id)
}

pub fn id_from_label(label: &str, conn: &SqliteConnection) -> ZiaResult<Option<i32>> {
    let label_s = label.to_string();
    let ids_of_text = match find_text(label_s, conn) 
                          { Ok(t) => t,
                           Err(e) => return Err(DBError::Query(e))};
    match ids_of_text.len() {0 => Ok(None),
                             1 => id_from_label_text_id(ids_of_text[0], conn),
                             _ => Err(DBError::Ambiguity("Don't know what to do when there is more than one concept that is the same text".to_string()))} 
}

fn id_from_label_text_id(id_of_text: i32, conn: &SqliteConnection) -> ZiaResult<Option<i32>> {
   let labelling_ids = try!(find_reduction(id_of_text, conn));
   match labelling_ids.len() {0 => Ok(None),
                              1 => select_argument(labelling_ids[0], LABEL, conn),
                              _ => Err(DBError::Ambiguity("Don't know what to do when there is more than one concept that reduces to the same text".to_string()))
                              }
}

pub fn label_from_id(id: i32, conn: &SqliteConnection) -> ZiaResult<Option<String>> {
    let label_definition_of_id = try!(find_definition(LABEL,id, conn));
    match label_definition_of_id {None => Ok(None),
                                  Some(def) => label_text_from_label_definition(def, conn)} 
}

fn label_text_from_label_definition(id: i32, conn: &SqliteConnection) -> ZiaResult<Option<String>> {
    let text_id = try!(find_normal_form(id, conn));
    match text_id {    None => Ok(None),
                   Some(id) => select_text(id,conn)}
}

pub fn refactor_id(id_before: i32, id_after: i32, luid: i32, conn: &SqliteConnection) -> QueryResult<()>{
    assert!(id_after < luid);
    assert!(id_before < luid);
    try!(update(text::table.filter(text::id.eq(id_before))).set(text::id.eq(id_after)).execute(conn));
    try!(update(integers::table.filter(integers::id.eq(id_before))).set(integers::id.eq(id_after)).execute(conn));
    try!(update(reductions::table.filter(reductions::id.eq(id_before))).set(reductions::id.eq(id_after)).execute(conn));
    try!(update(reductions::table.filter(reductions::normal_form.eq(id_before))).set(reductions::normal_form.eq(id_after)).execute(conn));
    try!(update(definitions::table.filter(definitions::id.eq(id_before))).set(definitions::id.eq(id_after)).execute(conn));  
    try!(update(definitions::table.filter(definitions::applicant.eq(id_before))).set(definitions::applicant.eq(id_after)).execute(conn)); 
    try!(update(definitions::table.filter(definitions::argument.eq(id_before))).set(definitions::argument.eq(id_after)).execute(conn));
    Ok(())        
}

/// Returns the id of the application of the given applicant id and argument id pair.
/// If the application is not defined for the given pair, this function inserts a definition
/// and assigns a new id.
pub fn insert_definition(applicant: i32, argument: i32, conn: &SqliteConnection) -> ZiaResult<i32> {
    let application = try!(find_definition(applicant, argument, conn));
    match application 
        {None => {let definition_id = try!(assign_new_id(conn));
                  let definition_result = (definitions::id.eq(definition_id),
                                           definitions::applicant.eq(applicant),
                                           definitions::argument.eq(argument));
                  try!(insert_into(definitions::table).values(&definition_result).execute(conn));
                  Ok(definition_id)}, 
         Some(id) => Ok(id)}
}

pub fn unlabel(id: i32, conn: &SqliteConnection) -> ZiaResult<()> {
    match try!(find_definition(LABEL, id, conn))
        {None => Ok(()),
         Some(application) => 
             match try!(find_normal_form(application, conn))
                  {None => Err(DBError::Absence("label has no reduction".to_string())),
                   Some(text_id) =>
                       {try!(delete(definitions::table.filter(definitions::id.eq(application))).execute(conn));
                        try!(delete(reductions::table.filter(reductions::id.eq(application))).execute(conn)); 
                        try!(delete(text::table.filter(text::id.eq(text_id))).execute(conn));
                        Ok(())
                        }
                   },
         }
}

// Functions that operate only on the integers table.

fn insert_integer(id: i32, result: i32, conn: &SqliteConnection) -> QueryResult<usize>{
    let integer_result = (integers::id.eq(id), integers::result.eq(result));
    insert_into(integers::table).values(&integer_result).execute(conn)
}

pub fn assign_new_id(connection: &SqliteConnection) -> ZiaResult<i32> {
    let luid = try!(select_integer(LUID, connection));
    try!(update_integer(LUID,luid+1, connection));
    Ok(luid)
    
}

pub fn select_integer(id: i32, conn: &SqliteConnection) -> ZiaResult<i32> {
    let integer_vec = try!(integers::table.filter(integers::id.eq(id)).select(integers::result).load::<(i32)>(conn));
    match integer_vec.len() 
        {0 => Err(DBError::Absence("Don't know what to do when an integer is undefined".to_string())),
         1 => Ok(integer_vec[0]),
         _ => Err(DBError::Ambiguity("Don't know what to do when an integer is ambiguously defined".to_string()))
         }
}

fn update_integer(id: i32, result: i32, conn: &SqliteConnection) -> QueryResult<usize>{
    update(integers::table.filter(integers::id.eq(id))).set(integers::result.eq(result)).execute(conn)
}

// Functions that operate only on the text table.

fn find_text(result: String , conn: &SqliteConnection) -> QueryResult<Vec<i32>> {
    text::table.filter(text::result.eq(result)).select(text::id).load::<(i32)>(conn)
}

fn insert_text(id: i32, result: &str, conn: &SqliteConnection) -> QueryResult<usize>{
    let text_result = (text::id.eq(id), text::result.eq(result));
    insert_into(text::table).values(&text_result).execute(conn)
}

fn select_text(id: i32, conn: &SqliteConnection) -> ZiaResult<Option<String>> {
    let text = try!(text::table.filter(text::id.eq(id)).select(text::result).load::<(String)>(conn));
    match text.len() {0 => Ok(None),
                      1 => Ok(Some(text[0].clone())),
                      _ => Err(DBError::Ambiguity("Don't know what to do when a concept has multiple text values".to_string()))}
}

// Functions that operate only on the reductions table.

pub fn insert_reduction3(id: i32, mut normal_form_id: i32, conn: &SqliteConnection) -> ZiaResult<usize> {
    let normal_form = try!(find_normal_form(id, conn));
    match normal_form 
        {   None => (),
         Some(_) => return Err(DBError::Redundancy(format!("Reduction rule already exists for concept {:?}",id)))};
    let prereductions = try!(find_reduction(id, conn));
    let postreduction = try!(find_normal_form(normal_form_id, conn));
    match postreduction {None => (), Some(n) => normal_form_id = n};
    for prereduction in prereductions {
        try!(update_reduction(prereduction, normal_form_id, conn));
    }
    Ok(try!(insert_reduction2(id, normal_form_id, conn)))
}

pub fn find_normal_form(id: i32, conn: &SqliteConnection) -> ZiaResult<Option<i32>>{
    let reductions = try!(reductions::table.filter(reductions::id.eq(id)).select(reductions::normal_form).load::<(i32)>(conn));
    match reductions.len() {
        0 => Ok(None),
        1 => Ok(Some(reductions[0])),
        _ => Err(DBError::Ambiguity("Multiple reductions for the same concept".to_string()))
    }
}

fn find_reduction(normal_form: i32, conn: &SqliteConnection) -> QueryResult<Vec<i32>>{
    reductions::table.filter(reductions::normal_form.eq(normal_form)).select(reductions::id).load::<(i32)>(conn)
}

fn update_reduction(id: i32, new_normal_form: i32, conn: &SqliteConnection) -> QueryResult<usize> {
    update(reductions::table.filter(reductions::id.eq(id))).set(reductions::normal_form.eq(new_normal_form)).execute(conn)    
}

fn insert_reduction2(id: i32, normal_form_id: i32, conn: &SqliteConnection) -> QueryResult<usize> {
    let reduction_result = (reductions::id.eq(id), reductions::normal_form.eq(normal_form_id));
    insert_into(reductions::table).values(&reduction_result).execute(conn)
}

// Functions that operate only on the definitions table.

fn select_argument(id: i32, applicant: i32, conn: &SqliteConnection) -> ZiaResult<Option<i32>> {
    let arguments = try!(definitions::table.filter(definitions::id.eq(id).and(definitions::applicant.eq(applicant))).select(definitions::argument).load::<i32>(conn));
    match arguments.len() {0 => Ok(None),
                           1 => Ok(Some(arguments[0])),
                           _ => Err(DBError::Ambiguity("Multiple definitions with the same application id exist.".to_string()))}
}

pub fn find_definition(applicant:i32,argument:i32,conn:&SqliteConnection) -> ZiaResult<Option<i32>> {
    let applications = try!(definitions::table.filter(definitions::applicant.eq(applicant).and(definitions::argument.eq(argument))).select(definitions::id).load::<i32>(conn));
    match applications.len() {0 => Ok(None),
                              1 => Ok(Some(applications[0])),
                              _ => Err(DBError::Ambiguity("Multiple definitions with the same applicant and argument pair exist.".to_string()))}
}

pub fn select_definition(id: i32, conn: &SqliteConnection) -> ZiaResult<Option<(i32,i32)>> {
    let definitions = try!(definitions::table.filter(definitions::id.eq(id)).select((definitions::applicant,definitions::argument)).load::<(i32,i32)>(conn));
    match definitions.len() {0 => Ok(None),
                             1 => Ok(Some(definitions[0])),
                             _ => Err(DBError::Ambiguity("More than one definition for the same id".to_string()))} 
}

