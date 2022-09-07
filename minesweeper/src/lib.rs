pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut s_list = vec![];
    // Add everything to a vector
    for (i, val) in minefield.iter().enumerate() {
        let row_width = val.len();
        let mut accumulator = 3;

        // In a row_width == 1, situation you only want to look back one step
        // Or you end up counting incorrectly
        if row_width == 1 {
            accumulator = 0;
        }


        // Essentially what this does it, searches -1, 0, 1 from one level above
        // And -1 from the same level
        // Adding mines to values behind itself in the array as it comes accross them
        // + adds mines found from behind to the current x,y
        for (ii, value) in val.as_bytes().iter().enumerate() {
            let mut found_mines = 0;
            for x in 0..=accumulator {
                // Check for small width
                if row_width < 2 && x == 2 {
                    break;
                }

                // Cast to isize since usize cant go below 0

                // Honestly afterwards this is mucho complicado to explain
                // Felt much more intuitive on paper
                let mut result_of_calc = i as isize * row_width as isize - (row_width as isize - ii as isize) + row_width as isize - 1 as isize;
                if x < 2 {
                    result_of_calc = i as isize * row_width as isize - (row_width as isize - ii as isize) - x as isize;
                } else if x == 2 {
                    result_of_calc = i as isize * row_width as isize - (row_width as isize - ii as isize) + 1 as isize;
                }

                // We dont want to search from two i:s up, only max one
                let dont_search_below = (i.saturating_sub(1)) * row_width;


                // List cant have values under 0, so check
                if result_of_calc >= 0 {
                    match s_list.get(result_of_calc as usize) {
                        Some(v) => {
                            // If index we are looking at is a mine
                            if *v == -1 {
                                if result_of_calc >= dont_search_below as isize {
                                    if ii != 0 || x != 3 {
                                        found_mines += 1;
                                    }
                                }
                                
                            // If we are currently on a mine
                            } else if value.to_string() == "42" {
                                if result_of_calc >= dont_search_below as isize { 
                                    if ii + 1 != row_width || x != 2 {
                                        if ii != 0 || x != 3 { 
                                            s_list[result_of_calc as usize] += 1;
                                        }
                                    }
                                }
                            }
                        },
                        None => (),
                    }
                }
            }

            // If current value is a mine, we push - 1 
            // else we push all mines we found from behind to it
            if value.to_string() == "42" {
                // -1 equals *
                s_list.push(-1);
            } else {
                s_list.push(found_mines);
            }
        }
    }

    // Here we construct the string array such as: ["**", "**"]
    // From the currently existing stack like formation such as: [-1, -1, -1, -1]
    let mut string_list = vec![];
    let mut string_to_push = "".to_string();
    println!("{}", s_list.len());
    for v in s_list {
        if string_to_push.len() != minefield[0].len() {
            if v == -1 {
                string_to_push = format!("{}{}", string_to_push, "*");
            } else if v == 0 {
                string_to_push = format!("{}{}", string_to_push, " ");
            } else {
                string_to_push = format!("{}{}", string_to_push, v);
            }
        }

        if string_to_push.len() == minefield[0].len()  {
            string_list.push(string_to_push);
            string_to_push = "".to_string();
        }
    }

    // Very much an afterthought, needed for no_columns test 
    if minefield.len() > string_list.len() {
        string_list.push("".to_string());
    }
    string_list
}
