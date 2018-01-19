use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
enum Status {
    Dead,
    Stun,
    Poison,
}

struct Battler {
    name: String,
    hp: i32,
    maxhp: i32,
    mp: i32,
    maxmp: i32,
    status: HashMap<Status, usize>,
}

impl Battler {
    fn damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp < 0 {
            self.cure_all_status();
            self.status.insert(Status::Dead, 1);
        }
    }

    fn cure_all_status(&mut self) {
        self.status.remove(&Status::Stun);
        self.status.remove(&Status::Poison);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn die_for_me() {
        let mut battler = Battler{
            name: String::from("test-battler"),
            hp: 10,
            maxhp: 100,
            mp: 1,
            maxmp: 10,
            status: HashMap::new(),
        };

        battler.damage(10);

        assert_eq!(battler.hp, 0);
    }
}
