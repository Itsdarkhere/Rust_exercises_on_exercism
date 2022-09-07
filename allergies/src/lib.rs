pub struct Allergies;

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {

    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        
    }
}
