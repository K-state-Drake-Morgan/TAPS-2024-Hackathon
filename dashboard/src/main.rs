use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

pub enum PlotMessage {
    HelloWorld,
    None,
}

#[derive(Properties, PartialEq)]
pub struct PlotProps {}

pub struct Plot {
    canvas: NodeRef,
}

impl Component for Plot {
    type Message = PlotMessage;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(PlotMessage::HelloWorld);
        Plot {
            canvas : NodeRef::default(),
        } 
    }

    // _ tecnically means that we don't use ctx at all, if you need it remove the _
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlotMessage::HelloWorld => {
            
            let element: HtmlCanvasElement = self.canvas.cast().unwrap();
            let parent = element.parent_element().unwrap();
            let rect = parent.get_bounding_client_rect();

            element.set_height(rect.height() as u32);
            element.set_width(rect.width() as u32);

            let backend = CanvasBackend::with_canvas_object(element).unwrap();
            
            let drawing_area = backend.into_drawing_area();
            drawing_area.fill(&RGBColor(200,200,200)).unwrap();
                        
            let mut chart = ChartBuilder::on(&drawing_area)
                .caption("y=x^2", ("sans-serif", 14).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32).unwrap();
            
            chart.configure_mesh().draw().unwrap();
            
            chart
                .draw_series(LineSeries::new(
                    (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                    &RED,
                )).unwrap()
                .label("y = x^2")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
            },
            _ => {},
        } true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! (
          <div>
            <canvas ref = {self.canvas.clone()}/>
          </div>
        )
    }
    
    type Properties = PlotProps;
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Header />
            <Body />
            <Footer />
        </>
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header>
            <h1>{ "TAPS" }</h1>
        </header>
    }
}

#[function_component(Body)]
fn body() -> Html {
    html! {
        <main>
            <div class="sidebar">
                <Sidebar />
            </div>
            <div class="info">
                <Canvas />
            </div>
        </main>
    }
}

#[function_component(Sidebar)]
fn sidebar() -> Html {
    html! {
        <div class="sidebar-content">
            <h2>{ "Sidebar" }</h2>
            <ul>
                <li>{ "Item 1" }</li>
                <li>{ "Item 2" }</li>
                <li>{ "Item 3" }</li>
            </ul>
        </div>
    }
}

#[function_component(Canvas)]
fn canvas() -> Html {
    html! {
        <Plot />
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer>
            <p>{ "Footer content goes here" }</p>
        </footer>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
