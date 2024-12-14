use std::rc::Rc; // Used for reference-counted shared ownership
use std::cell::RefCell; // Enables mutable borrowing in an immutable Rc
use yew::prelude::*; // Yew framework prelude for building web components
mod blockchain; // Blockchain module containing logic and data structures
use blockchain::Blockchain; // Importing the Blockchain struct
use wasm_bindgen::JsCast; // For working with JavaScript DOM types

/// Main application component
#[function_component(App)]
fn app() -> Html {
    // State to hold the blockchain instance, wrapped in Rc<RefCell> for shared mutable access
    let blockchain = use_state(|| Rc::new(RefCell::new(Blockchain::new())));

    // State to hold the result messages displayed to the user
    let result = use_state(|| String::from("Welcome to Blockchain B"));

    // Callback for creating a new account
    let on_create_account = {
        let blockchain = blockchain.clone(); // Clone to move into the closure
        let result = result.clone();
        Callback::from(move |(id, balance): (String, i64)| {
            let mut chain = blockchain.borrow_mut(); // Borrow mutably to update the blockchain
            result.set(chain.create_account(id, balance)); // Update the result with the outcome
        })
    };

    // Callback for transferring funds between accounts
    let on_transfer = {
        let blockchain = blockchain.clone();
        let result = result.clone();
        Callback::from(move |(from, to, amount): (String, String, i64)| {
            let mut chain = blockchain.borrow_mut();
            result.set(chain.transfer(from, to, amount));
        })
    };

    // Callback for retrieving an account's balance
    let on_get_balance = {
        let blockchain = blockchain.clone();
        let result = result.clone();
        Callback::from(move |id: String| {
            let chain = blockchain.borrow(); // Borrow immutably to fetch balance
            result.set(chain.get_balance(&id));
        })
    };

    // HTML for the application
    html! {
        <>
            // Main container styling and layout
            <div
                style="
                    font-family: Arial, sans-serif; 
                    color: #ffffff; 
                    background: linear-gradient(135deg, #1e1e2f, #2b2b40); 
                    height: 100vh; 
                    display: flex; 
                    justify-content: center; 
                    align-items: center; 
                    text-align: center;
                "
            >
                // Inner container with additional styling
                <div
                    class="container"
                    style="
                        background: rgba(255, 255, 255, 0.1); 
                        border-radius: 16px; 
                        backdrop-filter: blur(10px); 
                        box-shadow: 0 4px 30px rgba(0, 0, 0, 0.5); 
                        padding: 20px; 
                        max-width: 400px;
                        width: 90%;
                    "
                >
                    <h1 style="color: #4CAF50; font-size: 32px; margin-bottom: 20px;">{ "Blockchain B" }</h1>

                    // Result display section
                    <div 
                        class="result"
                        style="
                            margin: 20px 0; 
                            font-size: 18px; 
                            color: #E6E6E6; 
                            padding: 10px; 
                            border: 1px solid rgba(255, 255, 255, 0.2); 
                            border-radius: 8px;
                        "
                    >
                        { (*result).clone() } // Display the current result message
                    </div>

                    // Section for creating a new account
                    <div style="margin-bottom: 20px;">
                        <h2 style="color: #2196F3; margin-bottom: 10px;">{ "Create Account" }</h2>
                        <input 
                            id="create-id"
                            style="
                                margin: 5px; 
                                padding: 10px; 
                                width: calc(100% - 20px); 
                                border: 1px solid rgba(255, 255, 255, 0.3); 
                                border-radius: 8px; 
                                background: rgba(0, 0, 0, 0.3); 
                                color: #ffffff;
                            " 
                            placeholder="Account ID"
                        />
                        <input 
                            id="create-balance"
                            style="
                                margin: 5px; 
                                padding: 10px; 
                                width: calc(100% - 20px); 
                                border: 1px solid rgba(255, 255, 255, 0.3); 
                                border-radius: 8px; 
                                background: rgba(0, 0, 0, 0.3); 
                                color: #ffffff;
                            " 
                            type="number" 
                            placeholder="Initial Balance"
                        />
                        <button 
                            style="
                                margin: 10px; 
                                padding: 10px 20px; 
                                background-color: rgba(72, 219, 251, 0.7); 
                                color: white; 
                                border: none; 
                                border-radius: 8px; 
                                cursor: pointer; 
                                font-weight: bold; 
                                width: calc(100% - 20px);
                            "
                            onclick={
                                let on_create_account = on_create_account.clone();
                                Callback::from(move |_| {
                                    let id = get_input_value("create-id"); // Fetch input value for account ID
                                    let balance = get_input_value("create-balance").parse().unwrap_or(0); // Parse input value for balance
                                    on_create_account.emit((id, balance)); // Trigger the create account callback
                                })
                            }>
                            { "Create" }
                        </button>
                    </div>

                    // Section for transferring funds
                    <div style="margin-bottom: 20px;">
                        <h2 style="color: #FF5722; margin-bottom: 10px;">{ "Transfer Funds" }</h2>
                        <input 
                            id="from-id"
                            style="
                                margin: 5px; 
                                padding: 10px; 
                                width: calc(100% - 20px); 
                                border: 1px solid rgba(255, 255, 255, 0.3); 
                                border-radius: 8px; 
                                background: rgba(0, 0, 0, 0.3); 
                                color: #ffffff;
                            " 
                            placeholder="From ID"
                        />
                        <input 
                            id="to-id"
                            style="
                                margin: 5px; 
                                padding: 10px; 
                                width: calc(100% - 20px); 
                                border: 1px solid rgba(255, 255, 255, 0.3); 
                                border-radius: 8px; 
                                background: rgba(0, 0, 0, 0.3); 
                                color: #ffffff;
                            " 
                            placeholder="To ID"
                        />
                        <input 
                            id="transfer-amount"
                            style="
                                margin: 5px; 
                                padding: 10px; 
                                width: calc(100% - 20px); 
                                border: 1px solid rgba(255, 255, 255, 0.3); 
                                border-radius: 8px; 
                                background: rgba(0, 0, 0, 0.3); 
                                color: #ffffff;
                            " 
                            type="number" 
                            placeholder="Amount"
                        />
                        <button 
                            style="
                                margin: 10px; 
                                padding: 10px 20px; 
                                background-color: rgba(255, 87, 34, 0.7); 
                                color: white; 
                                border: none; 
                                border-radius: 8px; 
                                cursor: pointer; 
                                font-weight: bold; 
                                width: calc(100% - 20px);
                            "
                            onclick={
                                let on_transfer = on_transfer.clone();
                                Callback::from(move |_| {
                                    let from = get_input_value("from-id");
                                    let to = get_input_value("to-id");
                                    let amount = get_input_value("transfer-amount").parse().unwrap_or(0);
                                    on_transfer.emit((from, to, amount)); // Trigger the transfer callback
                                })
                            }>
                            { "Transfer" }
                        </button>
                    </div>

                    // Section for checking balance
                    <div style="margin-bottom: 20px;">
                        <h2 style="color: #673AB7; margin-bottom: 10px;">{ "Get Balance" }</h2>
                        <input 
                            id="balance-id"
                            style="
                                margin: 5px; 
                                padding: 10px; 
                                width: calc(100% - 20px); 
                                border: 1px solid rgba(255, 255, 255, 0.3); 
                                border-radius: 8px; 
                                background: rgba(0, 0, 0, 0.3); 
                                color: #ffffff;
                            " 
                            placeholder="Account ID"
                        />
                        <button 
                            style="
                                margin: 10px; 
                                padding: 10px 20px; 
                                background-color: rgba(103, 58, 183, 0.7); 
                                color: white; 
                                border: none; 
                                border-radius: 8px; 
                                cursor: pointer; 
                                font-weight: bold; 
                                width: calc(100% - 20px);
                            "
                            onclick={
                                let on_get_balance = on_get_balance.clone();
                                Callback::from(move |_| {
                                    let id = get_input_value("balance-id");
                                    on_get_balance.emit(id); // Trigger the get balance callback
                                })
                            }>
                            { "Check Balance" }
                        </button>
                    </div>
                </div>
            </div>
        </>
    }
}

/// Helper function to retrieve input value by element ID
fn get_input_value(id: &str) -> String {
    web_sys::window() // Get the browser's window object
        .unwrap()
        .document() // Get the document object
        .unwrap()
        .get_element_by_id(id) // Locate the element by ID
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>() // Cast to an HtmlInputElement
        .unwrap()
        .value() // Retrieve the input's value
}

/// Entry point for the application
fn main() {
    yew::Renderer::<App>::new().render(); // Render the App component
}
