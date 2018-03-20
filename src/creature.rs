pub struct Creature {
    creatures: Vec<*some_data*>
}

impl Creature {
    fn generate_creature(room: Room, m: module_name) {
        m.generate(room)
    }

    pub fn new(room: Room) -> Creature {
        Creature {
            creatures: creature.generate_creature(room, creature_generator),
        }
    }
}
