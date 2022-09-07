pub fn translate(input: &str) -> String {
    // 'a' | 'e' | 'i' | 'o' | 'u'
    let mut result_s = String::new();

    for string in input.split(" ") {
        // Reset variables on each iteration
        let mut count_c = 0;
        let mut count_v = 0;
        let mut new_start = String::new();
        let mut new_end = String::new();
        let mut is_exception = false;

        for x in string.chars() {
            match x {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    if count_c == 0 {
                        count_v += 1;
                        new_end += "ay";
                        break;
                    } else {
                        // Check for rule 3
                        if new_end.ends_with('q') && x == 'u' && count_v == 0 {
                            new_end.push(x);
                            count_v += 1;
                        } else {
                            count_v += 1;
                            new_start.push(x);
                        }
                    }
                },
                'a'..='z' | 'A'..='Z' => {
                    if count_v == 0 {
                        // Rule 4
                        if x == 'y' && count_c > 1 {
                            new_start.push(x);
                            count_c += 1;
                            // y acts like an vowel, so... we trigger the else
                            count_v += 1;

                        // Please note that "xr" and "yt" at the beginning of a word make vowel sounds
                        } else if (new_end == "y" && x == 't') || (new_end == "x" && x == 'r') {
                            is_exception = true;
                            break;
                        } else {
                            count_c += 1;
                            new_end.push(x);
                        }
                    } else {
                        count_c += 1;
                        new_start.push(x);
                    }
                },
                _ => ()
            }
        }

        if count_v == 1 && count_c == 0 {
            // Add space
            if result_s.len() > 0 {
                result_s += " ";
            } 
            result_s += &format!("{}{}", input, new_end)
        // Triggered on xr and yt
        } else if is_exception {
            // Add space
            if result_s.len() > 0 {
                result_s += " ";
            } 
            result_s += &format!("{}{}", input, "ay");
        } else {
            new_end += "ay";

            // Add space
            if result_s.len() > 0 {
                result_s += " ";
            } 
            result_s += &format!("{}{}", new_start, new_end);
        }
    }

    result_s
}
