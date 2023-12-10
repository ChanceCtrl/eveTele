mod graph1d;
mod graph2d2y;

gtk::glib::wrapper! {
    pub struct Graph2D2Y(ObjectSubclass<graph2d2y::Graph2D2Y>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

gtk::glib::wrapper! {
    pub struct Graph1D(ObjectSubclass<graph1d::Graph1D>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Graph2D2Y {
    pub fn new() -> Self {
        gtk::glib::Object::builder().build()
    }
}

impl Graph1D {
    pub fn new() -> Self {
        gtk::glib::Object::builder().build()
    }
}
