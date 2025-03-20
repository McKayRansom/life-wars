use std::rc::Rc;



struct Quad {
    level: i32,
    nw: Rc<Quad>,
    ne: Rc<Quad>,
    se: Rc<Quad>,
    sw: Rc<Quad>,
}

impl Quad {

    const alive: Quad = Quad {
        level: 0,
        // nw: Rc::unin
        ne: todo!(),
        se: todo!(),
        sw: todo!(),
    }

    fn new(nw: Rc<Quad>, ne: Rc<Quad>, se: Rc<Quad>, sw: Rc<Quad>) -> Self {
        Self {
            level: nw.level + 1,
            nw,
            ne,
            se,
            sw,
        }
    }
}
