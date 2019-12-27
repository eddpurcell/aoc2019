pub type Vector = (i64, i64, i64);

#[derive(PartialEq)]
pub struct Mass {
    pub pos: Vector,
    pub vel: Vector,
}

impl Mass {
    pub fn new(pos: Vector) -> Self {
        Self {
            pos,
            vel: (0, 0, 0),
        }
    }

    pub fn apply_gravity(&mut self, pos: (i64, i64, i64)) {
        if self.pos.0 < pos.0 {
            self.vel.0 += 1;
        } else if self.pos.0 > pos.0 {
            self.vel.0 -= 1;
        }
        if self.pos.1 < pos.1 {
            self.vel.1 += 1;
        } else if self.pos.1 > pos.1 {
            self.vel.1 -= 1;
        }
        if self.pos.2 < pos.2 {
            self.vel.2 += 1;
        } else if self.pos.2 > pos.2 {
            self.vel.2 -= 1;
        }
    }

    pub fn apply_velocity(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }

    fn potential_energy(&self) -> i64 {
        self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs()
    }

    pub fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}
