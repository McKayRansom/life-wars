use std::rc::Rc;

struct Quad {
    level: i32,
    nw: Option<Rc<Quad>>,
    ne: Option<Rc<Quad>>,
    se: Option<Rc<Quad>>,
    sw: Option<Rc<Quad>>,
}

impl Quad {
    const ALIVE: Quad = Quad {
        level: 0,
        nw: None,
        ne: None,
        se: None,
        sw: None,
    };

    const DEAD: Quad = Quad {
        level: 0,
        nw: None,
        ne: None,
        se: None,
        sw: None,
    };

    fn new(
        nw: Option<Rc<Quad>>,
        ne: Option<Rc<Quad>>,
        se: Option<Rc<Quad>>,
        sw: Option<Rc<Quad>>,
    ) -> Self {
        Self {
            level: 1, // nw.level + 1,
            nw,
            ne,
            se,
            sw,
        }
    }
}
