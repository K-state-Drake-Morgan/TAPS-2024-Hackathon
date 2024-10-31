use std::io::Cursor;

// Import necessary libraries for plotting, web canvas, and Yew framework
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use reqwest;
use wasm_bindgen_futures;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use gloo::console; // For logging messages to the browser console
use calamine::{open_workbook_auto_from_rs, RangeDeserializerBuilder, Reader}; // parse xlsx files

// Enum to define the different plot messages that can trigger a plot update
pub enum PlotMessage {
    HelloWorld,
    ByeWorld,
    None,
}

// Enum to handle messages in the App component
pub enum Message {
    UpdatePlot(PlotMessage), // Trigger an update to the plot based on the selected message
    None
}

// Main application structure containing a reference to the canvas
pub struct App {
    plot: NodeRef // NodeRef for accessing the canvas element
}

// Implement the Component trait for the App struct
impl Component for App {
    type Message = Message; // Define the message type for component events
    type Properties = (); // No properties are needed for this component

    // Function to create the App component instance
    fn create(_ctx: &Context<Self>) -> Self {
        App {
            plot: NodeRef::default() // Initialize NodeRef for the canvas
        }
    }

    // Function to handle updates based on incoming messages
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::UpdatePlot(plot_message) => {
                // Handle the specific plot messages to draw the graph
                match plot_message {
                    PlotMessage::HelloWorld => {
                        // Log message to console for debugging
                        console::log!("Hello!");

                        wasm_bindgen_futures::spawn_local(async { let _ = fetch_data().await; });

                        // Get the canvas element from the NodeRef
                        let element: HtmlCanvasElement = self.plot.cast().unwrap();
                        let parent = element.parent_element().unwrap();
                        
                        // Set canvas dimensions based on the parent element's size
                        let rect = parent.get_bounding_client_rect();
                        element.set_height(rect.height() as u32);
                        element.set_width(rect.width() as u32);

                        // Initialize the backend for plotting using the canvas element
                        let backend = CanvasBackend::with_canvas_object(element).unwrap();
                        
                        // Create a drawing area for the plot
                        let drawing_area = backend.into_drawing_area();
                        drawing_area.fill(&RGBColor(200, 200, 200)).unwrap(); // Fill background with light gray
                                    
                        // Build the chart with specific configurations
                        let mut chart = ChartBuilder::on(&drawing_area)
                            .caption("y=x^2", ("sans-serif", 14).into_font()) // Set title and font
                            .margin(5) // Set margins for the chart
                            .x_label_area_size(30) // Space for x-axis labels
                            .y_label_area_size(30) // Space for y-axis labels
                            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32).unwrap(); // Define the axis ranges
                        
                        // Configure and draw the mesh/grid of the chart
                        chart.configure_mesh().draw().unwrap();
                        
                        // Draw the series for y = x^2
                        chart.draw_series(LineSeries::new(
                            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)), // Calculate points
                            &BLUE, // Color of the line
                        )).unwrap()
                        .label("y = x^2") // Label for the legend
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED)); // Legend line element

                    },
                    PlotMessage::ByeWorld => {
                        // Log message to console for debugging
                        console::log!("Bye!");

                        // Similar logic as HelloWorld, but for ByeWorld
                        let element: HtmlCanvasElement = self.plot.cast().unwrap();
                        let parent = element.parent_element().unwrap();
                        let rect = parent.get_bounding_client_rect();
                        
                        element.set_height(rect.height() as u32);
                        element.set_width(rect.width() as u32);

                        let backend = CanvasBackend::with_canvas_object(element).unwrap();
                        
                        let drawing_area = backend.into_drawing_area();
                        drawing_area.fill(&RGBColor(200, 200, 200)).unwrap();
                                    
                        let mut chart = ChartBuilder::on(&drawing_area)
                            .caption("y=x^2", ("sans-serif", 14).into_font())
                            .margin(5)
                            .x_label_area_size(30)
                            .y_label_area_size(30)
                            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32).unwrap();
                        
                        chart.configure_mesh().draw().unwrap();
                        
                        // Draw the series for y = x^2 but with negative x-values
                        chart.draw_series(LineSeries::new(
                            (-50..=50).map(|x| -x as f32 / 50.0).map(|x| (x, x * x)), // Inverted x-values
                            &RED, // Color of the line
                        )).unwrap()
                        .label("y = x^2") // Label for the legend
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED)); // Legend line element

                    },
                    PlotMessage::None => {}, // No action for None message
                }
                true // Indicate that the state has changed
            },
            Message::None => false, // No action needed
        }
    }

    // Function to define the view layout of the App component
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <header>
                    <h1>{ "TAPS" }</h1> // Main title of the application
                </header>
                <main>
                    <div class="sidebar"> // Sidebar for buttons
                        <ul>
                            <li>
                                // Button to update the plot to HelloWorld
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld))}>{ "Hello World" }</button>
                            </li>
                            <li>
                                // Button to update the plot to ByeWorld
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::ByeWorld))}>{ "Bye World" }</button>
                            </li>
                        </ul>
                    </div>
                    <div class="information"> // Container for the canvas
                        <canvas ref={self.plot.clone()} /> // Canvas element for plotting
                    </div>
                </main>
                <footer>
                    <p>{ "Footer" }</p> // Placeholder for footer content
                </footer>
            </>
        }
    }
}

async fn fetch_data() -> Result<Vec<(String, f64)>, String> {
    console::log!("Fetching DATA!");
    let file = reqwest::get("http://127.0.0.1:3000/TAPS-2024-Hackathon/data/sensor_data/24 KSU TAPS AquaSpy.xlsx").await.unwrap();
    let bytes = file.bytes().await.unwrap();
    let bytes_wrapper = Cursor::new(bytes);

    console::log!("Data Sharing to workbook");
    let mut workbook = open_workbook_auto_from_rs(bytes_wrapper).map_err(|e| e.to_string())?;
    let range = workbook.worksheet_range("Team #12 Data").expect("Sheet not found!");
    
    console::log!("Deserializing");
    let deserializer = RangeDeserializerBuilder::new().from_range(&range).map_err(|e| e.to_string())?;
    let mut data = Vec::new();

    console::log!("Resulting!");
    for result in deserializer {
        let (name, value): (String, f64) = result.map_err(|e| e.to_string()).unwrap_or(("BAD".to_string(), 0.0));
        data.push((name, value));
    }

    console::log!("Printing data!");
    console::log!(format!("{:?}", data));

    Ok(data)
}

// Entry point of the application
fn main() {
    // Render the App component in the Yew framework
    yew::Renderer::<App>::new().render();
}
