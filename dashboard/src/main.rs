use std::{io::Cursor, sync::{Arc, Mutex}};

// Import necessary libraries for plotting, web canvas, and Yew framework
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use reqwest;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use gloo::console; // For logging messages to the browser console
use calamine::{open_workbook_auto_from_rs, DataType, Reader}; // parse xlsx files

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
    plot: NodeRef, // NodeRef for accessing the canvas element
    plot_data: Vec<(f32, f32)>
}

// Implement the Component trait for the App struct
impl Component for App {
    type Message = Message; // Define the message type for component events
    type Properties = (); // No properties are needed for this component

    // Function to create the App component instance
    fn create(_ctx: &Context<Self>) -> Self {
        App {
            plot: NodeRef::default(), // Initialize NodeRef for the canvas
            plot_data: Vec::new()
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

                        let app = Arc::new(Mutex::new(App{plot: NodeRef::default(), plot_data: Vec::new()}));
                        let app_f = Arc::clone(&app);

                        wasm_bindgen_futures::spawn_local( async move {
                            App::fetch_data(app_f.lock().unwrap()).await;
                        });

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
                        
                        let min_x = app.lock().unwrap().plot_data.clone().into_iter().min_by_key(|x| x.0 as u64).unwrap().0;
                        let max_x = app.lock().unwrap().plot_data.clone().into_iter().max_by_key(|x| x.0 as u64).unwrap().0;

                        let min_y = app.lock().unwrap().plot_data.clone().into_iter().min_by_key(|x| x.1 as u64).unwrap().1;
                        let max_y = app.lock().unwrap().plot_data.clone().into_iter().max_by_key(|x| x.1 as u64).unwrap().1;

                        // Build the chart with specific configurations
                        let mut chart = ChartBuilder::on(&drawing_area)
                            .caption("y=x^2", ("sans-serif", 14).into_font()) // Set title and font
                            .margin(5) // Set margins for the chart
                            .x_label_area_size(30) // Space for x-axis labels
                            .y_label_area_size(30) // Space for y-axis labels
                            .build_cartesian_2d(min_x..max_x, min_y..max_y).unwrap(); // Define the axis ranges
                        
                        // Configure and draw the mesh/grid of the chart
                        chart.configure_mesh().draw().unwrap();
                        
                        // Draw the series for y = x^2
                        chart.draw_series(LineSeries::new(
                            app.lock().unwrap().plot_data.clone(), // Calculate points
                            &BLUE, // Color of the line
                        )).unwrap()
                        .label("y = x^2") // Label for the legend
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED)); // Legend line element

                    },
                    PlotMessage::ByeWorld => {},
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

impl App {
    async fn fetch_data(mut state: std::sync::MutexGuard<'_, App>) {
        console::log!("Fetching data...");

        // Fetch the file
        let response = reqwest::get("http://127.0.0.1:3000/TAPS-2024-Hackathon/data/sensor_data/24 KSU TAPS AquaSpy.xlsx")
            .await
            .map_err(|e| format!("Failed to fetch data: {}", e)).unwrap();
            
        let bytes = response.bytes()
            .await
            .map_err(|e| format!("Failed to read response bytes: {}", e)).unwrap();
        
        let bytes_wrapper = Cursor::new(bytes);
        console::log!("Data shared to workbook...");

        // Open the workbook
        let mut workbook = open_workbook_auto_from_rs(bytes_wrapper)
            .map_err(|e| format!("Failed to open workbook: {}", e)).unwrap();
        
        // Get the specified worksheet
        let range = workbook.worksheet_range("Team #12 Data").unwrap();

        console::log!("Deserializing data...");
        let mut data = Vec::new();

        // Iterate through rows
        for row in range.rows() {
            if row.len() > VALUE_COLUMN {
                let name = row[NAME_COLUMN].clone();
                let a: Option<f64> = match name {
                    calamine::Data::Int(x) => Some(x as f64),
                    calamine::Data::Float(x) => Some(x),
                    calamine::Data::String(_x) => None,
                    calamine::Data::Bool(_x) => None,
                    calamine::Data::DateTime(excel_date_time) => Some(excel_date_time.as_f64()),
                    calamine::Data::DateTimeIso(_x) => None,
                    calamine::Data::DurationIso(_x) => None,
                    calamine::Data::Error(_cell_error_type) => None,
                    calamine::Data::Empty => None,
                };
                let keyn = a.unwrap_or(0.0) as f32;
                let value = calamine::DataType::get_float(&row[VALUE_COLUMN]).unwrap_or(0.0) as f32;
                data.push((keyn, value));
            }
        }

        console::log!("Data retrieved successfully!");
        console::log!(format!("{:?}", data));

        state.plot_data = data;
    }
}

// Assume X is the index for the name and Y is for the value
const NAME_COLUMN: usize = 0 /* index for column X */;
const VALUE_COLUMN: usize = 1 /* index for column Y */;


// Entry point of the application
fn main() {
    // Render the App component in the Yew framework
    yew::Renderer::<App>::new().render();
}
