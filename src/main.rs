extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::prelude::*;
use relm::Relm;
use relm::Update;
use relm::Widget;

#[derive(Msg)]
pub enum Msg {
    Draw(cairo::Context),
    Click((f64, f64)),
    Quit,
}

pub struct Win {
    window: gtk::Window,
    maparea: gtk::DrawingArea,
}

impl Update for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;

    fn model(_relm: &Relm<Self>, config: ()) -> () {
        config
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Click((x, y)) => self.maparea.queue_draw_area(x as i32, y as i32, 10, 10),
            Msg::Draw(context) => {
                let (x, y, _x, _y) = context.clip_extents();
                context.paint();
                context.set_source_rgb(1., 0., 0.);
                context.set_line_width(2.);
                context.rectangle(x, y, 10., 10.);
                context.fill();
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Self {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_gravity(gdk::Gravity::Center);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(800, 600);
        window.set_title(&format!("Relmap v{}", env!("CARGO_PKG_VERSION")));
        let maparea = create_drawing_area();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), gtk::Inhibit(false))
        );

        connect!(
            relm,
            maparea,
            connect_button_press_event(_, ev),
            return (Some(Msg::Click(ev.get_position())), gtk::Inhibit(false))
        );

        {
            let relm = relm.clone();
            maparea.connect_draw(move |_, c| {
                let context = c.clone();
                let (x, y, _x, _y) = context.clip_extents();
                context.paint();
                context.set_source_rgb(0., 1., 0.5);
                context.set_line_width(2.);
                context.rectangle(x, y, 10., 10.);
                context.fill();
                relm.stream().emit(Msg::Draw(context));
                gtk::Inhibit(false)
            });
        }
        // */

        window.add(&maparea);
        window.show_all();

        Win { window, maparea }
    }
}

fn create_drawing_area() -> gtk::DrawingArea {
    let area = gtk::DrawingArea::new();
    area.set_hexpand(true);
    area.set_vexpand(true);

    area.set_events(
        area.get_events() | gdk::EventMask::POINTER_MOTION_MASK.bits() as i32
            | gdk::EventMask::BUTTON_PRESS_MASK.bits() as i32,
    );

    area
}

fn main() {
    Win::run(()).unwrap();
}
