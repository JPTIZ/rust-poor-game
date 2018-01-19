mod rpg {
    enum Status {
        dead(usize),
        stun(usize),
        poison(usize),
    }

    struct Battler {
        name: String,
        hp: i32,
        maxhp: i32,
        mp: i32,
        maxmp: i32,
        status: Status,
    }

    impl Battler {
        fn damage(&self, damage: i32) {
            self.hp -= damage;
            if self.hp < 0 {
                self.remove_status();
                self.status.dead = 1;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Battler;

        #[test]
        fn die_for_me() {
            let mut battler = Battler{
                name: "test-battler",
                hp: 10,
                maxhp: 100,
                mp: 1,
                maxmp: 10,
                status: Status{},
            }

            battler.damage(10);

            assert_eq!(battler.hp, 0);
        }
    }
}
