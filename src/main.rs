use sdmt_lab_2::linked_list::List;

fn main() {
    println!("Creating a new empty list");
    let mut list = List::new();
    println!("List: {:?}", list);
    println!("Length: {}", list.length());

    println!("\nAppending characters 'a', 'b', 'c', 'd' to the list");
    list.append('a');
    list.append('b');
    list.append('c');
    list.append('d');
    println!("List: {:?}", list);
    println!("Length: {}", list.length());

    println!("\nInserting 'x' at index 0 (beginning)");
    list.insert('x', 0).unwrap();
    println!("List: {:?}", list);

    println!("\nInserting 'y' at index 3 (middle)");
    list.insert('y', 3).unwrap();
    println!("List: {:?}", list);

    println!("\nInserting 'z' at index {} (end)", list.length());
    list.insert('z', list.length()).unwrap();
    println!("List: {:?}", list);

    println!("\nTrying to insert at invalid index (should fail)");
    match list.insert('!', 100) {
        Ok(_) => println!("Insertion succeeded"),
        Err(e) => println!("Insertion failed: {}", e),
    }

    println!("\nGetting element at index 0");
    match list.get(0) {
        Ok(elem) => println!("Element at index 0: '{}'", elem),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nGetting element at index 3");
    match list.get(3) {
        Ok(elem) => println!("Element at index 3: '{}'", elem),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nTrying to get element at invalid index");
    match list.get(100) {
        Ok(elem) => println!("Element at index 100: '{}'", elem),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nFinding first occurrence of 'b'");
    let first_b = list.find_first('b');
    if first_b == -1 {
        println!("'b' not found");
    } else {
        println!("First 'b' found at index: {}", first_b);
    }

    println!("\nAdding another 'b'");
    list.append('b');
    println!("List: {:?}", list);

    println!("\nFinding last occurrence of 'b'");
    let last_b = list.find_last('b');
    if last_b == -1 {
        println!("'b' not found");
    } else {
        println!("Last 'b' found at index: {}", last_b);
    }

    println!("\nFinding first occurrence of 'q' (not in the list)");
    let first_q = list.find_first('q');
    if first_q == -1 {
        println!("'q' not found");
    } else {
        println!("First 'q' found at index: {}", first_q);
    }

    println!("\nDeleting element at index 1");
    match list.delete(1) {
        Ok(deleted) => {
            println!("Deleted element: '{}'", deleted);
            println!("List: {:?}", list);
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\nAdding multiple 'a's");
    list.append('a');
    list.append('a');
    println!("List: {:?}", list);

    println!("\nDeleting all occurrences of 'a'");
    list.delete_all('a');
    println!("List: {:?}", list);

    println!("\nReversing the list");
    list.reverse();
    println!("Reversed list: {:?}", list);

    println!("\nCreating a second list");
    let mut list2 = List::new();
    list2.append('1');
    list2.append('2');
    list2.append('3');
    println!("Second list: {:?}", list2);

    println!("\nCloning the second list");
    let mut list2_clone = list2.clone();
    println!("Cloned list: {:?}", list2_clone);

    println!("\nAdding '4' and '5' to the cloned list");
    list2_clone.append('4');
    list2_clone.append('5');
    println!("Second list: {:?}", list2);
    println!("Cloned list: {:?}", list2_clone);

    println!("\nExtending first list with second list");
    list.extend(&list2);
    println!("Extended list: {:?}", list);
    println!("Length of extended list: {}", list.length());

    println!("\nClearing the second list");
    list2.clear();
    println!("Cleared list: {:?}", list2);
    println!("Length of cleared list: {}", list2.length());
}
