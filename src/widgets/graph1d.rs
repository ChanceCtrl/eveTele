use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use plotters::prelude::*;
use plotters_cairo::CairoBackend;

use std::cell::Cell;
use std::error::Error;
// use std::f64;

#[derive(Debug, Default, glib::Properties)]
#[properties(wrapper_type = super::Graph1D)]
pub struct Graph1D {
    #[property(get, set, minimum = 0.0, maximum = 1024.0)]
    value: Cell<f64>,
}

#[glib::object_subclass]
impl ObjectSubclass for Graph1D {
    const NAME: &'static str = "Graph1D";
    type Type = super::Graph1D;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
        crate::widgets::Graph1D::ensure_type();
    }
}

impl ObjectImpl for Graph1D {
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
    }
}

impl WidgetImpl for Graph1D {
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
        self.plot_def(backend).unwrap();
    }
}

impl Graph1D {
    pub fn plot_def<'a, DB: DrawingBackend + 'a>(
        &self,
        backend: DB,
    ) -> Result<(), Box<dyn Error + 'a>> {
        // Define drawing area
        let root = backend.into_drawing_area();

        // Try to fill with black
        root.fill(&WHITE)?;

        // Define 'chart'
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption("Pot Val", ("sans-serif", 15.0).into_font())
            .build_cartesian_2d(0f32..1f32, 0f32..1024f32)?;

        // Configure thy chart
        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_axes()
            .draw()?;

        // Draw first value
        chart.draw_series(LineSeries::new(
            (0..=100).map(|x| (x as f32 / 10.0, self.value.get() as f32)),
            &BLUE,
        ))?;

        root.present()?;
        Ok(())
    }
}
