use std::sync::{Arc, Mutex};
use yew::prelude::*;
mod blockchain;
use blockchain::Blockchain;
use std::thread;
use std::time::Duration;

#[function_component(App)]
fn app() -> Html {
    let blockchain: UseStateHandle<Arc<Mutex<Blockchain>>> = use_state(|| Blockchain::new());
    let result = use_state(|| String::from("Welcome to Blockchain App"));

    // Callback for creating a new account
    let on_create_account = {
        let blockchain = blockchain.clone();
        let result = result.clone();
        Callback::from(move |(id, balance): (String, i64)| {
            let mut chain = blockchain.lock().unwrap();
            result.set(chain.create_account(id, balance));
        })
    };

    // Callback for transferring funds between accounts
    let on_transfer = {
        let blockchain = blockchain.clone();
        let result = result.clone();
        Callback::from(move |(from, to, amount): (String, String, i64)| {
            let mut chain = blockchain.lock().unwrap();
            result.set(chain.transfer(from, to, amount));
        })
    };

    // Callback for retrieving an account's balance
    let on_get_balance = {
        let blockchain = blockchain.clone();
        let result = result.clone();
        Callback::from(move |id: String| {
            let chain = blockchain.lock().unwrap();
            result.set(chain.get_balance(&id));
        })
    };

    html! {
        <div
            style="
                font-family: Arial, sans-serif; 
                background: linear-gradient(135deg, #1e1e2f, #2b2b40); 
                height: 100vh; 
                display: flex; 
                justify-content: center; 
                align-items: center; 
                text-align: center; 
                color: #ffffff;
            "
        >
            <div
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
                <h1 style="color: #4CAF50; font-size: 32px; margin-bottom: 20px;">{ "Blockchain App" }</h1>

                <div 
                    style="
                        margin: 20px 0; 
                        font-size: 18px; 
                        color: #E6E6E6; 
                        padding: 10px; 
                        border: 1px solid rgba(255, 255, 255, 0.2); 
                        border-radius: 8px;
                    "
                >
                    { (*result).clone() }
                </div>

                <div style="margin-bottom: 20px;">
                    <h2 style="color: #2196F3; margin-bottom: 10px;">{ "Create Account" }</h2>
                    <input 
                        id="create-id" 
                        placeholder="Account ID"
                        style="
                            margin: 5px; 
                            padding: 10px; 
                            width: calc(100% - 20px); 
                            border: 1px solid rgba(255, 255, 255, 0.3); 
                            border-radius: 8px; 
                            background: rgba(0, 0, 0, 0.3); 
                            color: #ffffff;
                        "
                    />
                    <input 
                        id="create-balance" 
                        placeholder="Initial Balance" 
                        type="number"
                        style="
                            margin: 5px; 
                            padding: 10px; 
                            width: calc(100% - 20px); 
                            border: 1px solid rgba(255, 255, 255, 0.3); 
                            border-radius: 8px; 
                            background: rgba(0, 0, 0, 0.3); 
                            color: #ffffff;
                        "
                    />
                    <button 
                        onclick={
                            let on_create_account = on_create_account.clone();
                            Callback::from(move |_| {
                                let id = get_input_value("create-id");
                                let balance = get_input_value("create-balance").parse().unwrap_or(0);
                                on_create_account.emit((id, balance));
                            })
                        }
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
                    >
                        { "Create Account" }
                    </button>
                </div>

                <div style="margin-bottom: 20px;">
                    <h2 style="color: #FF5722; margin-bottom: 10px;">{ "Transfer Funds" }</h2>
                    <input 
                        id="from-id" 
                        placeholder="From ID"
                        style="
                            margin: 5px; 
                            padding: 10px; 
                            width: calc(100% - 20px); 
                            border: 1px solid rgba(255, 255, 255, 0.3); 
                            border-radius: 8px; 
                            background: rgba(0, 0, 0, 0.3); 
                            color: #ffffff;
                        "
                    />
                    <input 
                        id="to-id" 
                        placeholder="To ID"
                        style="
                            margin: 5px; 
                            padding: 10px; 
                            width: calc(100% - 20px); 
                            border: 1px solid rgba(255, 255, 255, 0.3); 
                            border-radius: 8px; 
                            background: rgba(0, 0, 0, 0.3); 
                            color: #ffffff;
                        "
                    />
                    <input 
                        id="transfer-amount" 
                        placeholder="Amount" 
                        type="number"
                        style="
                            margin: 5px; 
                            padding: 10px; 
                            width: calc(100% - 20px); 
                            border: 1px solid rgba(255, 255, 255, 0.3); 
                            border-radius: 8px; 
                            background: rgba(0, 0, 0, 0.3); 
                            color: #ffffff;
                        "
                    />
                    <button 
                        onclick={
                            let on_transfer = on_transfer.clone();
                            Callback::from(move |_| {
                                let from = get_input_value("from-id");
                                let to = get_input_value("to-id");
                                let amount = get_input_value("transfer-amount").parse().unwrap_or(0);
                                on_transfer.emit((from, to, amount));
                            })
                        }
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
                    >
                        { "Transfer Funds" }
                    </button>
                </div>

                <div style="margin-bottom: 20px;">
                    <h2 style="color: #673AB7; margin-bottom: 10px;">{ "Get Balance" }</h2>
                    <input 
                        id="balance-id" 
                        placeholder="Account ID"
                        style="
                            margin: 5px; 
                            padding: 10px; 
                            width: calc(100% - 20px); 
                            border: 1px solid rgba(255, 255, 255, 0.3); 
                            border-radius: 8px; 
                            background: rgba(0, 0, 0, 0.3); 
                            color: #ffffff;
                        "
                    />
                    <button 
                        onclick={
                            let on_get_balance = on_get_balance.clone();
                            Callback::from(move |_| {
                                let id = get_input_value("balance-id");
                                on_get_balance.emit(id);
                            })
                        }
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
                    >
                        { "Get Balance" }
                    </button>
                </div>
            </div>
        </div>
    }
}

use wasm_bindgen::JsCast;

/// Helper function to retrieve input value by element ID
fn get_input_value(id: &str) -> String {
    web_sys::window()
        .expect("No global `window` exists")
        .document()
        .expect("Should have a document on the window")
        .get_element_by_id(id)
        .expect(&format!("Element with id `{}` not found", id))
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect(&format!("Failed to cast element with id `{}` to HtmlInputElement", id))
        .value()
}

/// Entry point for the application
fn main() {
    yew::Renderer::<App>::new().render();
    
}
