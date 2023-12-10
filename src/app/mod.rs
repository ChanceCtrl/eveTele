use gtk::glib::clone;
use gtk::{prelude::*, Scale};
use gtk::{
    Application, ApplicationWindow, Button, DropDown, HeaderBar, Label, Stack, StackSwitcher,
};

use crate::background::ReadVal;
// use crate::widgets::Graph2D2Y;
use crate::widgets::Graph1D;

pub struct AppStruct {
    pub window: ApplicationWindow,
    pub container: gtk::Grid,
    pub header: HeaderBar,
    pub stack_switch: StackSwitcher,
    pub stack: Stack,
}

impl AppStruct {
    pub fn new(app: &Application) {
        // Create a window and set the title
        let window = ApplicationWindow::builder()
            .application(app)
            .title("eveTele")
            .build();

        // Create a box to throw widgets in
        let grid = gtk::Grid::builder()
            .row_spacing(4)
            .column_spacing(4)
            .build();

        // let plotbox = gtk::Box::builder()
        //     .orientation(gtk::Orientation::Vertical)
        //     .build();

        // Create a plot to plot shit
        let graph_snap = Graph1D::new();

        // Test slider to adjust value of the plot
        let slider_boi = Scale::with_range(gtk::Orientation::Vertical, 0.0, 1024.0, 1.0);

        slider_boi.connect_value_changed(
            clone!(@strong graph_snap, @strong slider_boi => move |_| {
            graph_snap.set_value(slider_boi.value());
            graph_snap.queue_draw();
            }),
        );

        // plotbox.append(&graph_snap);
        grid.attach(&graph_snap, 0, 0, 48, 128);
        grid.attach(&slider_boi, 58, 0, 20, 128);

        // Make DropDown for serial ports
        let port_vec = ReadVal::list_ports();
        let v: Vec<_> = port_vec.iter().map(|s| s.as_str()).collect();
        let port_drop = DropDown::from_strings(v.as_slice());
        port_drop.set_margin_top(1);
        port_drop.set_margin_bottom(1);

        // Make Button to test the serial port
        let port_test = Button::builder()
            .margin_start(1)
            .margin_end(12)
            .margin_top(1)
            .margin_bottom(1)
            .label("Read selected port")
            .build();

        // Make label for HeaderBar
        let title = Label::new(Some("eveTelemtry or something idk"));

        // Make a header bar to throw the drop down and test into
        let header = HeaderBar::new();
        header.pack_start(&port_test);
        header.pack_end(&port_drop);
        header.set_title_widget(Some(&title));

        // Test the selected port on click
        port_test.connect_clicked(
            clone!(@strong port_test, @strong port_drop, @strong port_vec => move |_| {
            port_test.set_label("Testing, check terminal");
            println!("{}", port_drop.to_string());
            ReadVal::start_bg_read(port_vec[port_drop.selected() as usize].clone());
            }),
        );

        // Present window
        window.set_titlebar(Some(&header));
        window.set_child(Some(&grid));
        window.present();
    }
}
