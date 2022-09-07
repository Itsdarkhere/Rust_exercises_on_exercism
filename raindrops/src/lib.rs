pub fn raindrops(n: u32) -> String {
    let mut raindrop_sound = vec![];

    if n % 3 == 0 {
        raindrop_sound.push("Pling")
    }
    if n % 5 == 0 {
        raindrop_sound.push("Plang")
    }
    if n % 7 == 0 {
        raindrop_sound.push("Plong")
    }
    // Check if not divisible by any of them
    if raindrop_sound.len() == 0 {
        return n.to_string()
    }

    // Join and return
    raindrop_sound.join("")
}
