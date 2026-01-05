#[allow(unused)]
// INFO: A reminder of Rust's borrowing and moving rules
// Ownership is a system of rules that manages memory safely and efficiently without a garbage collector.
/*
Move:
1. Only happens with a type if it doesn't implement the Copy trait; otherwise it will be copied.
Types implementing Copy are plain-value types with no destructor and no ownership of resources, so they can be safely duplicated bit-by-bit (e.g. most stack-allocated plain values).

For example: Heap-owning types always move(by default); duplicating their heap data requires an explicit `clone()`, which performs a deep copy of the heap allocation (this is cloning, not copying). Heap-owning types(which implement Drop to free memory when the owner goes out of scope) can never be copied, they can just be cloned because to implement Copy trait they must NOT have destructors to clean up when they go out of scope and by definition Heap based data types have destructors(Drop) to free memory when they go out of scope.
    - E.g.:
        let s = String::from("hi"); // owner
        let r = &s;                // pointer
        When 's' goes out of scope then Drop runs and clears the heap memory. In the context of running the destructor: We don't care about thr pointer 'r' if it goes out of scope or not, because even if it does we'll simply remove the stack data, but Drop will only run if 's' goes out of scope.
- Remember, the rule is if Copy trait implemented or NOT, and NOT heap/stack based allocation they are categories of data.
2. Move simply means that the "ownership" has been transferred to a different variable and the previous binding is dead to the compiler unless we decide to re-initialize it with the same name and in that case too the binding is "shadowed" and will be treated as new.

Borrow:
1. When we want to use(read/modify) the value without actually transferring the ownership. It saves us from pointless returning of tuples in which one variable always accounts for the original owner so that it goes back to the scope again.
2. By definition since we are borrowing it, we use "references" to do so. Rust automatically dereferences for methods but NOT for associated functions (which don't contain the &Self type (or self shorthand) in their function signature) ,operators and expressions. We use '&' to denote references and '*' to dereference the values.
3. We can have multiple number of immutable references(reading only) at once and use them at the same time, i.e., in overlapping lifetimes whereas for mutable references only one is allowed in case of overlapping lifetimes. Multiple mutable references cannot have overlapping lifetimes. Either one(mutable reference) with multiple immutable ones given their lifetimes don't overlap or none at all. I remember it simply like this: You cannot read(immutable reference) while you're writing(mutable reference) because you'll never be sure what's the correct data at a given time if both happen simultaneously.
*/
use rand::{Rng, thread_rng};
use std::{cmp::max, collections::HashMap, i32, io};
pub fn ex1() {
    let mut v: Vec<u32> = Vec::new();
    for i in 1..thread_rng().gen_range(2..11) {
        v.push(i * thread_rng().gen_range(1..5));
    }

    println!("v: {v:?}");

    // find median and mode

    // mode
    let mut freq: HashMap<u32, i32> = HashMap::new();
    let mut max_count = i32::MIN;
    for i in &v {
        let count = freq.entry(*i).or_insert(0);
        *count += 1;
        max_count = max(*count, max_count);
    }
    let mut mode = 0;
    for (k, v) in &freq {
        if *v == max_count {
            mode = *k;
        }
    }

    println!("freq: {freq:?}");
    println!("mode: {mode}");

    // median
    // sort the vector
    v.sort();
    let median: u32;
    println!("sorted v: {v:?}");
    if v.len() % 2 == 0 {
        median = (v[(v.len() / 2) - 1] + v[v.len() / 2]) / 2;
    } else {
        median = v[v.len() / 2];
    }
    println!("median: {median}");
}

pub fn ex2() {
    let mut word = String::from("apple");
    let first_char = word.chars().nth(0);
    if let Some(x) = &first_char {
        if !"aeiou".contains(*x) {
            word.remove(0);
            word.push_str(&format!("-{}ay", *x));
            println!("{word}");
        } else {
            word.push_str(&format!("-hay"));
            println!("{word}");
        }
    }
}

pub fn ex3() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new(); // Dept. : [Name1, Name2....]
    loop {
        let mut option = String::new();
        println!("Choose Options:");
        println!("1. Enter the text to add people to departments.");
        println!("2. See the list of one department and their people.");
        println!("3. See the list of all departments and their people.");
        println!("4. Exit.");

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to get the option.");
        if option.is_empty() {
            println!("Please choose an option.");
            continue;
        }
        let option: u8 = match option.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please choose a valid option.");
                continue;
            }
        };
        match option {
            1 => {
                let mut command = String::new();
                println!("\nPlease enter the command:");
                io::stdin()
                    .read_line(&mut command)
                    .expect("Failed to get the command.");
                let mut name: String = String::new();
                let mut dept: String = String::new();
                let mut counter: u8 = 2;
                for word in command.split_whitespace().rev() {
                    if counter == 0 {
                        break;
                    }
                    if word.to_lowercase() != "add" && word.to_lowercase() != "to" {
                        if counter == 2 {
                            dept = String::from(word);
                            counter -= 1;
                        } else if counter == 1 {
                            name = String::from(word);
                            counter -= 1;
                        }
                    }
                }
                if dept.is_empty() || name.is_empty() {
                    println!("No valid department or name found in the command!");
                    continue;
                }
                map.entry(dept.to_lowercase())
                    .or_insert_with(Vec::new)
                    .push(name.to_lowercase());
                println!();
            }
            2 => {
                println!("Please enter the department name:");
                let mut dept = String::new();
                io::stdin()
                    .read_line(&mut dept)
                    .expect("Failed to get the department name.");
                dept = dept.trim().to_lowercase();
                if dept.is_empty() {
                    println!("Please enter the department name.");
                    continue;
                }
                match map.get(&dept) {
                    Some(names) => {
                        println!("\n{} people found.", names.len());
                        for (i, name) in names.iter().enumerate() {
                            if i == names.len() - 1 {
                                println!("{}. {name}", i + 1)
                            } else {
                                println!("{}. {name},", i + 1)
                            }
                        }
                        println!();
                        continue;
                    }
                    None => {
                        println!("0 people found.");
                        continue;
                    }
                }
            }
            3 => {
                println!("\n-------------------");
                println!("Department | Name");
                for (dept, names) in &map {
                    for name in names {
                        println!("{dept} | {name}");
                    }
                }
                println!("-------------------\n");
                continue;
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Please choose a valid option.");
            }
        }
    }
}
