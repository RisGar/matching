use std::collections::BTreeMap;

use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

type Course = String;
type Capacity = u8;
type Priority = u32;
type StudentId = u32;

type StudentPref = Vec<(Course, Priority)>;

type Preferences = BTreeMap<StudentId, StudentPref>;
type Matchings = BTreeMap<String, Vec<(StudentId, Priority)>>;

/// Available courses and their capacity
fn courses() -> BTreeMap<Course, Capacity> {
  BTreeMap::from([
    ("Mo_08_1".to_string(), 35),
    ("Mo_10_1".to_string(), 35),
    ("Mo_10_2".to_string(), 35),
    ("Mo_10_3".to_string(), 45),
    ("Mo_10_4".to_string(), 35),
    ("Mo_10_5".to_string(), 35),
    ("Mo_12_1".to_string(), 35),
    ("Mo_12_2".to_string(), 45),
    ("Mo_12_3".to_string(), 35),
    ("Mo_14_1".to_string(), 35),
    ("Mo_12_2".to_string(), 45),
    ("Mo_12_3".to_string(), 35),
    ("Mo_12_4".to_string(), 35),
    ("Mo_12_5".to_string(), 35),
  ])
}

fn generate_student() -> StudentPref {
  let mut rng = thread_rng();

  // Gen random courses
  let num_courses = rng.gen_range(1..=courses().len());
  let course_map = courses();
  let selected_courses = course_map.keys().choose_multiple(&mut rng, num_courses);

  // Gen random (P)rios with random (L)ots (PLLLL)
  let mut pref = selected_courses
    .into_iter()
    .map(|e| {
      (
        e.clone(),
        rng.gen_range(1..=5) * 10000 + rng.gen_range(0000..=9999),
      )
    })
    .collect::<Vec<_>>();

  // Sort ascending by prio
  pref.sort_by(|a, b| a.1.cmp(&b.1));

  pref
}

/// Genrate batch of students with preferences
fn generate_students(preferences: &mut Preferences) {
  (1..courses().values().map(|&n| n as StudentId).sum()).for_each(|id| {
    let student = generate_student();
    preferences.insert(id, student);
  });
}

/// Gale-Shapley algorithm
fn proposing(preferences: &mut Preferences, matchings: &mut Matchings) -> Vec<StudentId> {
  let backup_preferences = preferences.clone();

  let mut unmatched_students: Vec<StudentId> = vec![];

  while !preferences.is_empty() {
    let (id, mut pref) = preferences.pop_first().unwrap(); // unwrap as we know there is at least one student

    if let Some((course, prio)) = pref.pop()
    // TODO: unwrap as we know there is at least one prio
    {
      let &course_capacity = courses()
        .get(&course)
        .expect("Course declared in priorities could not be found.");

      let course_students: &mut Vec<(StudentId, Priority)> = matchings
        .get_mut(&course)
        .expect("matchings not initialised");

      if course_students.len() < course_capacity as usize {
        course_students.push((id, prio));
      } else {
        let lowest_student: (StudentId, Priority) = *course_students.last().unwrap();

        // Replace lowest student
        if lowest_student.1 < prio {
          course_students.pop();
          course_students.push((id, prio));

          let lowest_prefs: &StudentPref = backup_preferences.get(&lowest_student.0).unwrap();

          preferences.insert(lowest_student.0, lowest_prefs.clone());
        } else {
          preferences.insert(id, pref);
        }
      }

      // Sort matchings descending by prio
      matchings.iter_mut().for_each(|(_, students)| {
        students.sort_by(|a, b| b.1.cmp(&a.1));
      });
    } else {
      unmatched_students.push(id);
    }
  }

  unmatched_students
}

fn main() {
  // courses and their respective students
  let mut matchings: Matchings = courses().keys().map(|x| (x.clone(), vec![])).collect();

  // students and their preferences
  let mut preferences: Preferences = BTreeMap::new();

  // insert self
  preferences.insert(0, generate_student());

  generate_students(&mut preferences);

  let unmatched = proposing(&mut preferences, &mut matchings);

  matchings.iter().for_each(|(course, students)| {
    println!(
      "{}: {:?}",
      course,
      students.iter().map(|(id, _)| id).collect::<Vec<_>>()
    );
  });

  println!("Unmatched students: {:?} ({})", unmatched, unmatched.len());
}
