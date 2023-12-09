mod graph;

gtk::glib::wrapper! {
    pub struct Graph2D(ObjectSubclass<graph::Graph2D>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Graph2D {
    pub fn new() -> Self {
        gtk::glib::Object::builder().build()
    }
}

// pub struct CanIt {
//     // The ID its tracking
//     pub id: String,
//     // The Value from that ID
//     pub value: u32,
// }
//
// impl CanIt {
//     pub fn new(id: String) -> Self {
//         Self { id, value: 0 }
//     }
//
//     pub fn read(id: String) -> u32 {
//         0
//     }
// }
