pub(crate) const INDEX_TOP_LEFT: usize = 0;
pub(crate) const INDEX_TOP_RIGHT: usize = 1;
pub(crate) const INDEX_BOTTOM_LEFT: usize = 2;
pub(crate) const INDEX_BOTTOM_RIGHT: usize = 3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}

impl Coord {
    pub fn broadcast(n: f32) -> Self {
        Self { x: n, y: n }
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Div<f32> for Coord {
    type Output = Coord;

    fn div(self, factor: f32) -> Coord {
        Coord {
            x: self.x / factor,
            y: self.y / factor,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bounds {
    pub top_left: Coord,
    pub bottom_right: Coord,
}

impl Bounds {
    pub(crate) fn section_by_index(&self, index: usize) -> Bounds {
        match index {
            INDEX_TOP_LEFT => self.top_left(),
            INDEX_TOP_RIGHT => self.top_right(),
            INDEX_BOTTOM_LEFT => self.bottom_left(),
            INDEX_BOTTOM_RIGHT => self.bottom_right(),
            _ => unreachable!(),
        }
    }

    pub fn width(&self) -> f32 {
        self.bottom_right.x - self.top_left.x
    }

    pub fn height(&self) -> f32 {
        self.bottom_right.y - self.top_left.y
    }

    pub(crate) fn get_subsection_index(&self, coord: Coord) -> usize {
        let self_mid = self.mid();

        if coord.x < self_mid.x {
            if coord.y < self_mid.y {
                INDEX_TOP_LEFT
            } else {
                INDEX_BOTTOM_LEFT
            }
        } else {
            if coord.y < self_mid.y {
                INDEX_TOP_RIGHT
            } else {
                INDEX_BOTTOM_RIGHT
            }
        }
    }

    pub(crate) fn get_subsection_by_index(&self, index: usize) -> Bounds {
        match index {
            INDEX_TOP_LEFT => self.top_left(),
            INDEX_TOP_RIGHT => self.top_right(),
            INDEX_BOTTOM_LEFT => self.bottom_left(),
            INDEX_BOTTOM_RIGHT => self.bottom_right(),
            _ => unreachable!(),
        }
    }

    pub(crate) fn get_subsection(&self, coord: Coord) -> (Bounds, usize) {
        let idx = self.get_subsection_index(coord);
        let bounds = self.get_subsection_by_index(idx);
        (bounds, idx)
    }

    pub(crate) fn corners(&self) -> [Coord; 4] {
        [
            self.top_left,
            Coord {
                // top right
                x: self.bottom_right.x,
                y: self.top_left.y,
            },
            self.bottom_right,
            Coord {
                x: self.top_left.x,
                y: self.bottom_right.y,
            },
        ]
    }

    pub(crate) fn mid(&self) -> Coord {
        self.top_left + (self.bottom_right - self.top_left) / 2.0
    }

    pub(crate) fn top_left(&self) -> Bounds {
        let mid = self.mid();
        Bounds {
            top_left: self.top_left,
            bottom_right: mid,
        }
    }

    pub(crate) fn top_right(&self) -> Bounds {
        let mid = self.mid();
        Bounds {
            top_left: Coord {
                x: mid.x,
                y: self.top_left.y,
            },
            bottom_right: Coord {
                x: self.bottom_right.x,
                y: mid.y,
            },
        }
    }

    pub(crate) fn bottom_left(&self) -> Bounds {
        let mid = self.mid();
        Bounds {
            top_left: Coord {
                x: self.top_left.x,
                y: mid.y,
            },
            bottom_right: Coord {
                x: mid.x,
                y: self.bottom_right.y,
            },
        }
    }

    pub(crate) fn bottom_right(&self) -> Bounds {
        let mid = self.mid();
        Bounds {
            top_left: mid,
            bottom_right: self.bottom_right,
        }
    }
}

#[test]
fn validate_splits() {
    let bounds = Bounds {
        top_left: Coord { x: 0.0, y: 0.0 },
        bottom_right: Coord { x: 100.0, y: 100.0 },
    };

    assert_eq!(
        bounds.top_left(),
        Bounds {
            top_left: Coord { x: 0.0, y: 0.0 },
            bottom_right: Coord { x: 50., y: 50. }
        }
    );
    assert_eq!(
        bounds.bottom_left(),
        Bounds {
            top_left: Coord { x: 0.0, y: 50.0 },
            bottom_right: Coord { x: 50., y: 100. }
        }
    );
    assert_eq!(
        bounds.top_right(),
        Bounds {
            top_left: Coord { x: 50.0, y: 0.0 },
            bottom_right: Coord { x: 100., y: 50. }
        }
    );
    assert_eq!(
        bounds.bottom_right(),
        Bounds {
            top_left: Coord { x: 50.0, y: 50.0 },
            bottom_right: Coord { x: 100., y: 100. }
        }
    );
}

#[test]
fn sanity_check() {
    let bounds = Bounds {
        top_left: Coord { x: 500.0, y: 0.0 },
        bottom_right: Coord {
            x: 1000.0,
            y: 500.0,
        },
    };
    let point = Coord {
        x: 740.768493,
        y: 238.742157,
    };

    let (sub_bounds, idx) = bounds.get_subsection(point);
    assert_eq!(
        sub_bounds,
        Bounds {
            top_left: Coord { x: 500.0, y: 0.0 },
            bottom_right: Coord { x: 750.0, y: 250.0 }
        }
    );
    assert_eq!(idx, INDEX_TOP_LEFT);
}
