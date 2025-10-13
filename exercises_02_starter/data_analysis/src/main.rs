use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

const ENROLMENTS_PATH: &str = "enrolments.psv";

fn main() -> io::Result<()> {
    let file = File::open(ENROLMENTS_PATH)?;
    let reader = io::BufReader::new(file);

    let mut unique_students: HashSet<String> = HashSet::new();
    let mut course_students: HashMap<String, HashSet<String>> = HashMap::new();
    let mut student_wams: HashMap<String, f64> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() < 9 {
            continue;
        }

        let course = fields[0].to_string();
        let student = fields[1].to_string();
        let wam: f64 = fields[5].parse().unwrap_or(0.0);

        unique_students.insert(student.clone());

        course_students
            .entry(course)
            .or_insert_with(HashSet::new)
            .insert(student.clone());

        student_wams.insert(student, wam);
    }

    println!("Number of students: {}", unique_students.len());

    let mut most_course = None;
    let mut least_course = None;

    for (course, students) in &course_students {
        let count = students.len();
        match most_course {
            None => most_course = Some((course, count)),
            Some((_, c)) if count > c => most_course = Some((course, count)),
            _ => {}
        }
        match least_course {
            None => least_course = Some((course, count)),
            Some((_, c)) if count < c => least_course = Some((course, count)),
            _ => {}
        }
    }

    if let Some((c, n)) = most_course {
        println!("Most common course: {} with {} students", c, n);
    }
    if let Some((c, n)) = least_course {
        println!("Least common course: {} with {} students", c, n);
    }

    let total_wam: f64 = student_wams.values().sum();
    let avg_wam = total_wam / student_wams.len() as f64;
    println!("Average WAM: {:.2}", avg_wam);

    Ok(())
}
