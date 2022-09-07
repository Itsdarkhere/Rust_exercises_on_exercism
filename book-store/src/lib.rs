use std::collections::HashMap;

pub fn lowest_price(books: &[u32]) -> u32 {
    let discount_map = 
        HashMap::from([(0, 0.0), (2, 0.4), (3, 0.8), (4, 1.6), (5, 2.0)]);
    let mut book_map = HashMap::new();
    let mut groups = Vec::new();

    // Insert values into hashmap for easier lookup
    for book in books {
        let mut book_count = book_map.entry(book).or_insert(1);
        *book_count += 1;
    }

    // Insert size per group into groups vector
    for (_, v) in book_map {
        let mut count = 0;
        loop {
            match groups.get(count) {
                Some(_) => groups[count] += 1,
                None => groups.push(1),
            };
            count += 1;
            
            // Break loop when count == amount of certain book
            if count - 1 == v {
                break;
            }
        }
    }

    for x in groups.iter() {
        println!("X: {x}");
    }

    let res: f64 = groups.iter().map(|v| *v as f64 * (8.0 - discount_map.get(v).unwrap())).sum();
    println!("{res}");
    res as u32

}
