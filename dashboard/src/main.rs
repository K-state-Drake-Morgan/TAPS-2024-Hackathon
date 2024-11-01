use std::io::Cursor;

// Import necessary libraries for plotting, web canvas, and Yew framework
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use reqwest;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use gloo::console; // For logging messages to the browser console
use calamine::{open_workbook_auto_from_rs, Reader}; // parse xlsx files

// Enum to define the different plot messages that can trigger a plot update
pub enum PlotMessage {
    HelloWorld(String, String, usize, usize, String),
    None,
}

// Enum to handle messages in the App component
pub enum Message {
    UpdatePlot(PlotMessage), // Trigger an update to the plot based on the selected message
    MakePlot(String, Vec<(f32, f32)>),
    None
}

// Main application structure containing a reference to the canvas
pub struct App {
    plot: NodeRef, // NodeRef for accessing the canvas element
}

// Implement the Component trait for the App struct
impl Component for App {
    type Message = Message; // Define the message type for component events
    type Properties = (); // No properties are needed for this component

    // Function to create the App component instance
    fn create(_ctx: &Context<Self>) -> Self {
        App {
            plot: NodeRef::default(), // Initialize NodeRef for the canvas
        }
    }

    // Function to handle updates based on incoming messages
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::UpdatePlot(plot_message) => {
                // Handle the specific plot messages to draw the graph
                match plot_message {
                    PlotMessage::HelloWorld(x, y, z, a, b) => {
                        // Log message to console for debugging
                        console::log!("Hello!");

                        ctx.link().send_future(App::fetch_data(x, y, z, a, b));
                    },
                    PlotMessage::None => {}, // No action for None message
                }
                true // Indicate that the state has changed
            },
            Message::None => false,
            Message::MakePlot(caption, vec) => {
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
                        
                        let min_x = vec.clone().into_iter().min_by_key(|x| x.0 as u64).unwrap_or((f32::MAX, f32::MAX)).0;
                        let max_x = vec.clone().into_iter().max_by_key(|x| x.0 as u64).unwrap_or((f32::MIN, f32::MIN)).0;

                        let min_y = vec.clone().into_iter().min_by_key(|x| x.1 as u64).unwrap_or((f32::MAX, f32::MAX)).1;
                        let max_y = vec.clone().into_iter().max_by_key(|x| x.1 as u64).unwrap_or((f32::MIN, f32::MIN)).1;

                        console::log!(format!("Max x, y = ({max_x}, {max_y})\nMim x, y = ({min_x}, {min_y})"));

                        // Build the chart with specific configurations
                        let mut chart = ChartBuilder::on(&drawing_area)
                            .caption(caption, ("sans-serif", 14).into_font()) // Set title and font
                            .margin(5) // Set margins for the chart
                            .x_label_area_size(30) // Space for x-axis labels
                            .y_label_area_size(30) // Space for y-axis labels
                            .build_cartesian_2d(min_x..max_x, min_y - 1.0..max_y + 1.0).unwrap(); // Define the axis ranges
                        
                        // Configure and draw the mesh/grid of the chart
                        chart.configure_mesh().draw().unwrap();
                        
                        // Draw the series for y = x^2
                        chart.draw_series(AreaSeries::new(
                            vec.clone(),
                            0.0,
                            ShapeStyle{
                                color: BLUE.mix(0.6),
                                filled: false,
                                stroke_width: 2,
                            }
                        )).unwrap()
                        //.label("y = x^2") // Label for the legend
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED)); // Legend line element

                        drawing_area.present().unwrap();

                        true
            }, // No action needed
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
                                <h2>{ "Team 12" }</h2>
                            </li>
                            <hr />
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 2, "Team #12 Moisture at 4'' / time".to_string())))}>{ "Team #12 Moisture at 4'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 3, "Team #12 Moisture at 8'' / time".to_string())))}>{ "Team #12 Moisture at 8'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 4, "Team #12 Moisture at 12'' / time".to_string())))}>{ "Team #12 Moisture at 12'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 5, "Team #12 Moisture at 16'' / time".to_string())))}>{ "Team #12 Moisture at 16'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 6, "Team #12 Moisture at 20'' / time".to_string())))}>{ "Team #12 Moisture at 20'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 7, "Team #12 Moisture at 24'' / time".to_string())))}>{ "Team #12 Moisture at 24'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 8, "Team #12 Moisture at 28'' / time".to_string())))}>{ "Team #12 Moisture at 28'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 9, "Team #12 Moisture at 32'' / time".to_string())))}>{ "Team #12 Moisture at 32'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 10, "Team #12 Moisture at 36'' / time".to_string())))}>{ "Team #12 Moisture at 36'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 11, "Team #12 Moisture at 40'' / time".to_string())))}>{ "Team #12 Moisture at 40'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 12, "Team #12 Moisture at 44'' / time".to_string())))}>{ "Team #12 Moisture at 44'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 13, "Team #12 Moisture at 48'' / time".to_string())))}>{ "Team #12 Moisture at 48'' / time" }</button>
                            </li>
                            //<!-- EC buttons -->
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 14, "Team #12 EC at 4'' / time".to_string())))}>{ "Team #12 EC at 4'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 15, "Team #12 EC at 8'' / time".to_string())))}>{ "Team #12 EC at 8'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 16, "Team #12 EC at 12'' / time".to_string())))}>{ "Team #12 EC at 12'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 17, "Team #12 EC at 16'' / time".to_string())))}>{ "Team #12 EC at 16'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 18, "Team #12 EC at 20'' / time".to_string())))}>{ "Team #12 EC at 20'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 19, "Team #12 EC at 24'' / time".to_string())))}>{ "Team #12 EC at 24'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 20, "Team #12 EC at 28'' / time".to_string())))}>{ "Team #12 EC at 28'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 21, "Team #12 EC at 32'' / time".to_string())))}>{ "Team #12 EC at 32'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 22, "Team #12 EC at 36'' / time".to_string())))}>{ "Team #12 EC at 36'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 23, "Team #12 EC at 40'' / time".to_string())))}>{ "Team #12 EC at 40'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 24, "Team #12 EC at 44'' / time".to_string())))}>{ "Team #12 EC at 44'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 25, "Team #12 EC at 48'' / time".to_string())))}>{ "Team #12 EC at 48'' / time" }</button>
                            </li>
                            //<!-- Temperature buttons -->
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 26, "Team #12 Temperature at 4'' / time".to_string())))}>{ "Team #12 Temperature at 4'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 27, "Team #12 Temperature at 8'' / time".to_string())))}>{ "Team #12 Temperature at 8'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 28, "Team #12 Temperature at 12'' / time".to_string())))}>{ "Team #12 Temperature at 12'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 29, "Team #12 Temperature at 16'' / time".to_string())))}>{ "Team #12 Temperature at 16'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 30, "Team #12 Temperature at 20'' / time".to_string())))}>{ "Team #12 Temperature at 20'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 31, "Team #12 Temperature at 24'' / time".to_string())))}>{ "Team #12 Temperature at 24'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 32, "Team #12 Temperature at 28'' / time".to_string())))}>{ "Team #12 Temperature at 28'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 33, "Team #12 Temperature at 32'' / time".to_string())))}>{ "Team #12 Temperature at 32'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 34, "Team #12 Temperature at 36'' / time".to_string())))}>{ "Team #12 Temperature at 36'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 35, "Team #12 Temperature at 40'' / time".to_string())))}>{ "Team #12 Temperature at 40'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 36, "Team #12 Temperature at 44'' / time".to_string())))}>{ "Team #12 Temperature at 44'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #12 Data".to_string(), 0, 37, "Team #12 Temperature at 48'' / time".to_string())))}>{ "Team #12 Temperature at 48'' / time" }</button>
                            </li>

                            <hr />
                            <li>
                                <h2>{ "Team 16" }</h2>
                            </li>
                            <hr />

                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 2, "Team #16 Moisture at 4'' / time".to_string())))}>{ "Team #16 Moisture at 4'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 3, "Team #16 Moisture at 8'' / time".to_string())))}>{ "Team #16 Moisture at 8'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 4, "Team #16 Moisture at 12'' / time".to_string())))}>{ "Team #16 Moisture at 12'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 5, "Team #16 Moisture at 16'' / time".to_string())))}>{ "Team #16 Moisture at 16'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 6, "Team #16 Moisture at 20'' / time".to_string())))}>{ "Team #16 Moisture at 20'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 7, "Team #16 Moisture at 24'' / time".to_string())))}>{ "Team #16 Moisture at 24'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 8, "Team #16 Moisture at 28'' / time".to_string())))}>{ "Team #16 Moisture at 28'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 9, "Team #16 Moisture at 32'' / time".to_string())))}>{ "Team #16 Moisture at 32'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 10, "Team #16 Moisture at 36'' / time".to_string())))}>{ "Team #16 Moisture at 36'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 11, "Team #16 Moisture at 40'' / time".to_string())))}>{ "Team #16 Moisture at 40'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 12, "Team #16 Moisture at 44'' / time".to_string())))}>{ "Team #16 Moisture at 44'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 13, "Team #16 Moisture at 48'' / time".to_string())))}>{ "Team #16 Moisture at 48'' / time" }</button>
                            </li>
                            //<!-- EC buttons -->
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 14, "Team #16 EC at 4'' / time".to_string())))}>{ "Team #16 EC at 4'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 15, "Team #16 EC at 8'' / time".to_string())))}>{ "Team #16 EC at 8'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 16, "Team #16 EC at 12'' / time".to_string())))}>{ "Team #16 EC at 12'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 17, "Team #16 EC at 16'' / time".to_string())))}>{ "Team #16 EC at 16'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 18, "Team #16 EC at 20'' / time".to_string())))}>{ "Team #16 EC at 20'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 19, "Team #16 EC at 24'' / time".to_string())))}>{ "Team #16 EC at 24'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 20, "Team #16 EC at 28'' / time".to_string())))}>{ "Team #16 EC at 28'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 21, "Team #16 EC at 32'' / time".to_string())))}>{ "Team #16 EC at 32'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 22, "Team #16 EC at 36'' / time".to_string())))}>{ "Team #16 EC at 36'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 23, "Team #16 EC at 40'' / time".to_string())))}>{ "Team #16 EC at 40'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 24, "Team #16 EC at 44'' / time".to_string())))}>{ "Team #16 EC at 44'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 25, "Team #16 EC at 48'' / time".to_string())))}>{ "Team #16 EC at 48'' / time" }</button>
                            </li>
                            //<!-- Temperature buttons -->
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 26, "Team #16 Temperature at 4'' / time".to_string())))}>{ "Team #16 Temperature at 4'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 27, "Team #16 Temperature at 8'' / time".to_string())))}>{ "Team #16 Temperature at 8'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 28, "Team #16 Temperature at 12'' / time".to_string())))}>{ "Team #16 Temperature at 12'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 29, "Team #16 Temperature at 16'' / time".to_string())))}>{ "Team #16 Temperature at 16'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 30, "Team #16 Temperature at 20'' / time".to_string())))}>{ "Team #16 Temperature at 20'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 31, "Team #16 Temperature at 24'' / time".to_string())))}>{ "Team #16 Temperature at 24'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 32, "Team #16 Temperature at 28'' / time".to_string())))}>{ "Team #16 Temperature at 28'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 33, "Team #16 Temperature at 32'' / time".to_string())))}>{ "Team #16 Temperature at 32'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 34, "Team #16 Temperature at 36'' / time".to_string())))}>{ "Team #16 Temperature at 36'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 35, "Team #16 Temperature at 40'' / time".to_string())))}>{ "Team #16 Temperature at 40'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 36, "Team #16 Temperature at 44'' / time".to_string())))}>{ "Team #16 Temperature at 44'' / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS AquaSpy.xlsx".to_string(), "Team #16 Data".to_string(), 0, 37, "Team #16 Temperature at 48'' / time".to_string())))}>{ "Team #16 Temperature at 48'' / time" }</button>
                            </li>
                            <li>{ "24 KSU TAPS Arable" }</li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 1, "Team #16 Chlorophyll Index / time".to_string())))}>{ "Team #16 Chlorophyll Index / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 2, "Team #16 Arable Field Evapotranspiration (mm) / time".to_string())))}>{ "Team #16 Arable Field Evapotranspiration (mm) / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 3, "Team #16 Arable Canopy Evapotranspiration (mm) / time".to_string())))}>{ "Team #16 Chlorophyll IndexArable Canopy Evapotranspiration (mm) / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 4, "Team #16 Growing Degree Days / time".to_string())))}>{ "Team #16 Growing Degree Days / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 5, "Team #16 Accumulated Growing Degree Days / time".to_string())))}>{ "Team #16 Accumulated Growing Degree Days / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 6, "Team #16 NDVI / time".to_string())))}>{ "Team #16 NDVI / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 7, "Team #16 Minimum Relative Humidity / time".to_string())))}>{ "Team #16 Minimum Relative Humidity / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 8, "Team #16 Relative Humidity at Max Temp / time".to_string())))}>{ "Team #16 Relative Humidity at Max Temp / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 9, "Team #16 Relative Humidity at Min Temp / time".to_string())))}>{ "Team #16 Relative Humidity at Min Temp / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 10, "Team #16 Shortwave Downwelling Radiation / time".to_string())))}>{ "Team #16 Shortwave Downwelling Radiation / time" }</button>
                            </li>
                            <li>
                                <button onclick={ctx.link().callback(|_| Message::UpdatePlot(PlotMessage::HelloWorld("data/sensor_data/24 KSU TAPS Arable.xlsx".to_string(), "Team #16 Data".to_string(), 0, 11, "Team #16 Max Temp / time".to_string())))}>{ "Team #16 Max Temp / time" }</button>
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
    async fn fetch_data(relative_url: String, team: String, column_name: usize, column_value: usize, caption: String) -> Message {
        console::log!("Fetching data...");

        // Fetch the file
        let response = reqwest::get("https://k-state-drake-morgan.github.io/TAPS-2024-Hackathon/".to_string() + &relative_url)
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
        let range = workbook.worksheet_range(&team).unwrap();

        console::log!("Deserializing data...");
        let mut data = Vec::new();

        // Iterate through rows
        for row in range.rows() {
            if row.len() > column_value {
                let name = row[column_name].clone();
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
                let value = calamine::DataType::get_float(&row[column_value]).unwrap_or(0.0) as f32;
                if keyn == 0.0 && value == 0.0 {
                    continue; // BAD... but shouldn't effect too much data
                }
                data.push((keyn, value));
            }
        }

        console::log!("Data retrieved successfully!");
        console::log!(format!("{:?}", data));

        Message::MakePlot(caption, data)
    }
}

// Entry point of the application
fn main() {
    // Render the App component in the Yew framework
    yew::Renderer::<App>::new().render();
}
