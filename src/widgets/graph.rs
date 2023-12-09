use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use plotters::prelude::*;
use plotters_cairo::CairoBackend;

use std::cell::Cell;
use std::error::Error;
// use std::f64;

#[derive(Debug, Default, glib::Properties)]
#[properties(wrapper_type = super::Graph2D)]
pub struct Graph2D {
    #[property(get, set, minimum = 0, maximum = 1024)]
    min_max: Cell<u32>,
    // value: Cell<f64>,
}

#[glib::object_subclass]
impl ObjectSubclass for Graph2D {
    const NAME: &'static str = "Graph2D";
    type Type = super::Graph2D;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
        crate::widgets::Graph2D::ensure_type();
    }
}

impl ObjectImpl for Graph2D {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        Self::derived_set_property(self, id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        Self::derived_property(self, id, pspec)
    }

    fn constructed(&self) {
        self.parent_constructed();
        self.obj().vexpands();
    }
}

impl WidgetImpl for Graph2D {
    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        // get width & height
        let width = self.obj().width() as u32;
        let height = self.obj().height() as u32;

        // Fail if either are 0
        if width == 0 || height == 0 {
            return;
        }

        // Make box to draw in
        let bounds = gtk::graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
        let cr = snapshot.append_cairo(&bounds);
        let backend = CairoBackend::new(&cr, (width, height)).unwrap();

        // Draw in said box
        self.plot_pdf(backend).unwrap();
    }
}

impl Graph2D {
    pub fn plot_pdf<'a, DB: DrawingBackend + 'a>(
        &self,
        backend: DB,
    ) -> Result<(), Box<dyn Error + 'a>> {
        // Define the area to draw to
        let root = backend.into_drawing_area();

        // Try to fill it with white
        root.fill(&WHITE)?;

        // Try to create 'cartesian_2d' called "chart" and do some setup on it
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .right_y_label_area_size(40)
            .margin(5)
            .caption("Dual Y-Axis Example", ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(0f32..10f32, (0.1f32..1e10f32).log_scale())?
            .set_secondary_coord(0f32..10f32, -1.0f32..1.0f32);

        // Graph thy 1st value
        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .y_desc("Log Scale")
            .y_label_formatter(&|x| format!("{:e}", x))
            .draw()?;

        // Configure thy 2nd value
        chart
            .configure_secondary_axes()
            .y_desc("Linear Scale")
            .draw()?;

        // Draw first value
        chart
            .draw_series(LineSeries::new(
                (0..=100).map(|x| (x as f32 / 10.0, (1.02f32).powf(x as f32 * x as f32 / 10.0))),
                &BLUE,
            ))?
            .label("y = 1.02^x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

        // Draw 2nd value
        chart
            .draw_secondary_series(LineSeries::new(
                (0..=100).map(|x| (x as f32 / 10.0, (x as f32 / 5.0).sin())),
                &RED,
            ))?
            .label("y = sin(2x)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        // Make it look better
        chart
            .configure_series_labels()
            .background_style(RGBColor(128, 128, 128))
            .draw()?;

        // Present thy work
        root.present()?;
        Ok(())
    }
}
