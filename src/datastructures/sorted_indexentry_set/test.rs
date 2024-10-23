use super::*;

fn prepare_set() -> IndexSet {
    let mut set: IndexSet = IndexSet::new();

    set.add(&IndexEntry::new("gmail", 0)); // len 50
    set.add(&IndexEntry::new("yahoo", 60)); // len 30
    set.add(&IndexEntry::new("facebook", 50)); // len 10
    set.add(&IndexEntry::new("adobe", 90)); // len 10
    set.add(&IndexEntry::new("google", 100)); // len 100

    set
}

#[test]
fn test_initialization() {
    let set = prepare_set();
    let binary_data = set.serialize();
    let new_set = IndexSet::deserialize(&binary_data);

    assert_eq!(set, new_set);
}

#[test]
fn test_insert() {
    let set = prepare_set();

    println!("{:?}", set.data);

    assert_eq!(set.data[0].key, "adobe");
    assert_eq!(set.data[1].key, "facebook");
    assert_eq!(set.data[2].key, "gmail");
    assert_eq!(set.data[3].key, "google");
    assert_eq!(set.data[4].key, "yahoo");
}

#[test]
fn test_find_key() {
    let set = prepare_set();

    if let Some((index, _)) = set.find("adobe") {
        assert_eq!(index, 0);
    } else {
        panic!("Function returned None");
    }

    if let Some((index, _)) = set.find("google") {
        assert_eq!(index, 3);
    } else {
        panic!("Function returned None");
    }
}

#[test]
fn test_remove_key() {
    let mut set = prepare_set();

    set.remove("adobe", 10);

    if set.find("adobe").is_some() {
        panic!("Entry was not deleted by remove method.");
    }

    let (_, y_entry) = set.find("yahoo").expect("Entry not found.");
    assert_eq!(y_entry.value, 60); // Yahoo stays at 60

    let (_, g_entry) = set.find("google").expect("Entry not found.");
    assert_eq!(g_entry.value, 90); // 100 - 10

    set.remove("yahoo", 30);
    set.remove("gmail", 50);

    let (_, f_entry) = set.find("facebook").expect("Entry not found");
    assert_eq!(f_entry.value, 0); // 50 - 50

    let (_, g2_entry) = set.find("google").expect("Entry not found");
    assert_eq!(g2_entry.value, 10); // 100 - 10 - 30 - 50
                                    //
}
