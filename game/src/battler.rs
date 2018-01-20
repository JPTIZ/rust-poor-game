use std::collections::HashMap;
use std::cmp::{min, max};

use std::slice::Iter;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Dead,
    Stun,
    Poison,
}

impl Status {
    pub fn values() -> Iter<'static, Status> {
        static STATUS: [Status; 3] = [Status::Dead, Status::Stun, Status::Poison];
        STATUS.into_iter()
    }
}

pub struct Battler {
    pub name: String,
    hp: i32,
    pub maxhp: i32,
    mp: i32,
    pub maxmp: i32,
    status: HashMap<Status, usize>,
}

impl Battler {
    pub fn new(name: String, maxhp: i32, maxmp: i32) -> Battler {
        Battler{
            name: name,
            hp: maxhp,
            maxhp: maxhp,
            mp: maxmp,
            maxmp: maxmp,
            status: HashMap::new(),
        }
    }

    pub fn hp(&self) -> i32 {
        self.hp
    }

    pub fn mp(&self) -> i32 {
        self.mp
    }

    pub fn damage(&mut self, damage: i32) {
        if self.dead() {
            return;
        }

        self.hp -= damage;
        if self.hp <= 0 {
            self.cure_all_status();
            self.status.insert(Status::Dead, 1);
        }
        self.hp = min(self.maxhp, max(0, self.hp));
    }

    pub fn damage_mp(&mut self, damage: i32) {
        self.mp = min(self.maxmp, max(0, self.mp - damage));
    }

    pub fn cure_status(&mut self, status: Status) {
        self.status.remove(&status);
    }

    pub fn cure_all_status(&mut self) {
        for status in Status::values() {
            self.cure_status(*status);
        }
    }

    pub fn inflict(&mut self, status: Status) {
        self.status.insert(status, 1);
    }

    pub fn has_status(&self, status: Status) -> bool {
        self.status.contains_key(&status)
    }

    pub fn dead(&self) -> bool {
        self.has_status(Status::Dead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_battler() -> Battler {
        Battler::new(
            String::from("fooboy"),
            100,
            10,
        )
    }

    #[test]
    fn die_for_me() {
        let mut battler = sample_battler();

        let hp = battler.hp;
        battler.damage(hp);

        assert_eq!(battler.hp, 0);

        battler.damage(1);
        assert_eq!(battler.hp, 0);

        assert!(battler.has_status(Status::Dead));
    }

    #[test]
    fn apply_status() {
        let mut battler = sample_battler();

        for status in Status::values() {
            let s = &*status;
            battler.inflict(*s);
            assert!(battler.has_status(*status));
        }

        for status in Status::values() {
            battler.cure_status(*status);
            assert!(!battler.has_status(*status));
        }

        battler.cure_all_status();
        assert!(!battler.has_status(Status::Poison));
    }

    #[test]
    fn hit_the_dead() {
        let mut battler = sample_battler();

        let hp = battler.hp;
        battler.damage(hp);

        assert!(battler.dead());

        battler.damage(-1);
        assert_eq!(battler.hp, 0);
    }
}
