pub struct GraphCan {
    // The window to hold the data
    pub frame: gtk::Frame,
    // The bar component
    // pub graph: plotters,
    // The ID its tracking
    pub id: String,
    // The Value from that ID
    pub value: u32,
}

impl GraphCan {
    pub fn new(id: String) -> Self {
        Self {
            frame: { gtk::Frame::new("Test_Frame_Label".into()) },

            id,

            value: 0,
        }
    }
}
