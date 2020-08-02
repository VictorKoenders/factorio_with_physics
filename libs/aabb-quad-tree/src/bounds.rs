pub(crate) const INDEX_TOP_LEFT: usize = 0;
pub(crate) const INDEX_TOP_RIGHT: usize = 1;
pub(crate) const INDEX_BOTTOM_LEFT: usize = 2;
pub(crate) const INDEX_BOTTOM_RIGHT: usize = 3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
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

    pub(crate) fn get_subsection_for_bounds(&self, bounds: &Bounds) -> (Bounds, usize) {
        let self_mid = self.mid();
        let bounds_mid = bounds.mid();

        let (sub, index) = if bounds_mid.0 < self_mid.0 {
            if bounds_mid.1 < self_mid.1 {
                (self.top_left(), INDEX_TOP_LEFT)
            } else {
                (self.bottom_left(), INDEX_BOTTOM_LEFT)
            }
        } else {
            if bounds_mid.1 < self_mid.1 {
                (self.top_right(), INDEX_TOP_RIGHT)
            } else {
                (self.bottom_right(), INDEX_BOTTOM_RIGHT)
            }
        };

        (sub, index)
    }

    pub(crate) fn mid(&self) -> (f32, f32) {
        (
            self.left + (self.right - self.left) / 2.0,
            self.top + (self.bottom - self.top) / 2.0,
        )
    }

    pub(crate) fn top_left(&self) -> Bounds {
        let mid = self.mid();
        Bounds {
            left: self.left,
            right: mid.0,
            top: self.top,
            bottom: mid.1,
        }
    }

    pub(crate) fn top_right(&self) -> Bounds {
        let mid = self.mid();
        Bounds {
            left: mid.0,
            right: self.right,
            top: self.top,
            bottom: mid.1,
        }
    }

    pub(crate) fn bottom_left(&self) -> Bounds {
        let mid = self.mid();
        Bounds {
            left: self.left,
            right: mid.0,
            top: mid.1,
            bottom: self.bottom,
        }
    }

    pub(crate) fn bottom_right(&self) -> Bounds {
        let mid = self.mid();
        Bounds {
            left: mid.0,
            right: self.right,
            top: mid.1,
            bottom: self.bottom,
        }
    }
}

#[test]
fn validate_splits() {
    let bounds = Bounds {
        top: 0.0,
        bottom: 100.0,
        left: 0.0,
        right: 100.,
    };

    assert_eq!(
        bounds.top_left(),
        Bounds {
            top: 0.0,
            bottom: 50.0,
            left: 0.0,
            right: 50.0
        }
    );
    assert_eq!(
        bounds.bottom_left(),
        Bounds {
            top: 50.0,
            bottom: 100.0,
            left: 0.0,
            right: 50.0
        }
    );
    assert_eq!(
        bounds.top_right(),
        Bounds {
            top: 0.0,
            bottom: 50.0,
            left: 50.0,
            right: 100.0
        }
    );
    assert_eq!(
        bounds.bottom_right(),
        Bounds {
            top: 50.0,
            bottom: 100.0,
            left: 50.0,
            right: 100.0
        }
    );
}
