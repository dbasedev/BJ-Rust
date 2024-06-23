use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::io::{Read, stdin};
use std::rc::Rc;

#[derive(Debug)]
struct Person {
    id: u32,
    know_truth: bool,
}

impl Person {
    fn new(id: u32, know_truth: bool) -> Person {
        Person { id, know_truth }
    }
}

#[derive(Debug)]
struct Party {
    id: u32,
    assigned_people: HashMap<u32, Person>,
    truth_people: u32,
}


impl Party {
    fn new(id: u32) -> Party {
        Party {
            id,
            assigned_people: HashMap::new(),
            truth_people: 0
        }
    }

    fn attend_person(&mut self, person: Person) {
        if person.know_truth {
            self.truth_people += 1;
        }
        self.assigned_people.insert(person.id, person);
    }

    fn is_person_attend(&self, person: &Person) -> bool {
        self.assigned_people.contains_key(&person.id)
    }

    fn has_relationship(&self, other: &Self) -> bool {
        for (_, people) in self.assigned_people.iter() {
            if other.is_person_attend(people) {
                return true;
            }
        }
        false
    }

    fn truth_included(&self) -> bool {
        self.truth_people != 0
    }
}

#[derive(Debug)]
struct PartyOrganizer {
    edges: Vec<(Rc<Party>, Vec<Rc<Party>>)>
}

impl PartyOrganizer {
    fn new() -> PartyOrganizer {
        PartyOrganizer {
            edges: Vec::new(),
        }
    }
    fn add_party(&mut self, party: Party) {
        let wrapped_party = Rc::new(party);

        let mut new_relationships = Vec::new();

        for (from, tos) in self.edges.iter_mut() {
            if wrapped_party.has_relationship(from) {
                tos.push(wrapped_party.clone());
                new_relationships.push(from.clone());
            }
        }

        self.edges.push((wrapped_party, new_relationships));
    }

    fn find_can_lies(&self) -> u32 {
        let mut deque = VecDeque::new();

        let mut hash_map = self.edges
            .iter()
            .map(|(from, tos)| (from.id, (from, tos, false)))
            .collect::<HashMap<u32, _>>();

        for (party, _) in self.edges.iter() {
            if party.truth_included() {
                deque.push_back(party.id);
            }
        }

        while !deque.is_empty() {
            let x = deque.pop_front().unwrap();
            match hash_map.get_mut(&x) {
              Some((from, tos, can_tell_lie)) if !*can_tell_lie => {
                  *can_tell_lie = true;
                  for person in tos.iter() {
                      deque.push_back(person.id);
                  }
              },
              _ => continue,
            }
        }
        hash_map.into_iter().map(|(_, (_, _, can_tell_lie))| if can_tell_lie { 0 } else { 1 }).sum()
    }
}


fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut iterations = buffer.split('\n');

    iterations
        .next()
        .unwrap();

    let know_truths_people_id = iterations
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|x| x.trim().parse()).collect::<HashSet<u32>>();

    let mut i = 0;
    let mut organizer = PartyOrganizer::new();
    while let Some(line) = iterations.next() {
        let persons = line
            .split_ascii_whitespace()
            .skip(1)
            .filter_map(
                |x| match x.trim().parse() {
                    Ok(id) => Some(id),
                    Err(_) => None,
            })
            .map(|x| Person::new(x, know_truths_people_id.contains(&x)))
            .collect::<Vec<Person>>();

        if persons.len() == 0 {
            continue;
        }

        let mut party = Party::new(i);
        for person in persons {
            party.attend_person(person);
        }
        i += 1;
        organizer.add_party(party);
    }
    let answer = organizer.find_can_lies();
    println!("{}", answer);
}
