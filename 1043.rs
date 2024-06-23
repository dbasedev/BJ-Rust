use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::io::{Read, stdin};

#[derive(Debug, Eq)]
struct Person {
    id: u32,
    know_truth: bool,
}

impl Person {
    fn new(id: u32, know_truth: bool) -> Person {
        Person { id, know_truth }
    }
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


#[derive(Debug, Eq)]
struct Party {
    id: u32,
    assigned_people: HashSet<Person>,
    truth_people: u32,
}

impl Hash for Party {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Party {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Party {
    fn new(id: u32) -> Party {
        Party {
            id,
            assigned_people: HashSet::new(),
            truth_people: 0
        }
    }

    fn attend_person(&mut self, person: Person) {
        if person.know_truth {
            self.truth_people += 1;
        }

        self.assigned_people.insert(person);
    }

    fn is_person_attend(&self, person: &Person) -> bool {
        self.assigned_people.contains(&person)
    }

    fn has_relationship(&self, other: &Self) -> bool {
        for people in self.assigned_people.iter() {
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
    parties: HashMap<u32, Party>,
    relationships: HashMap<u32, Vec<u32>>
}

impl PartyOrganizer {
    fn new() -> PartyOrganizer {
        PartyOrganizer {
            parties: HashMap::new(),
            relationships: HashMap::new(),
        }
    }
    fn add_party(&mut self, party: Party) {
        self.relationships.insert(party.id, Vec::new());

        for (id, existed) in self.parties.iter() {
            if party.has_relationship(existed) {
                self.relationships.get_mut(id).unwrap().push(party.id);
                self.relationships.get_mut(&party.id).unwrap().push(*id);
            }
        }
        self.parties.insert(party.id, party);
    }

    fn find_can_lies(&self) -> u32 {
        let mut deque = VecDeque::new();

        let mut hash_map = self.parties
            .iter()
            .map(|(x, _)| (*x, false))
            .collect::<HashMap<u32, bool>>();

        for (x, party) in self.parties.iter() {
            if party.truth_included() {
                deque.push_back(*x);
            }
        }

        while !deque.is_empty() {
            let x = deque.pop_front().unwrap();
            match hash_map.get_mut(&x) {
              Some(y) if !*y => {
                  *y = true;
                  for person in self.relationships.get(&x).unwrap() {
                      deque.push_back(*person);
                  }
              },
              _ => continue,
            }
        }
        hash_map.into_iter().map(|(_, can_tell_lie)| if can_tell_lie { 0 } else { 1 }).sum()
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
