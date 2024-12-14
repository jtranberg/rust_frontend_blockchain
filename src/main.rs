use std::rc::Rc;
use std::cell::RefCell;
use yew::prelude::*;
mod blockchain;
use blockchain::Blockchain;
use wasm_bindgen::JsCast;

#[function_component(App)]
fn app() -> Html {
    let blockchain = use_state(|| Rc::new(RefCell::new(Blockchain::new())));
    let result = use_state(|| String::from("Welcome to Blockchain B"));

    let on_create_account = {
        let blockchain = blockchain.clone();
        let result = result.clone();
        Callback::from(move |(id, balance): (String, i64)| {
            let mut chain = blockchain.borrow_mut();
            result.set(chain.create_account(id, balance));
        })
    };

    let on_transfer = {
        let blockchain = blockchain.clone();
        let result = result.clone();
        Callback::from(move |(from, to, amount): (String, String, i64)| {
            let mut chain = blockchain.borrow_mut();
            result.set(chain.transfer(from, to, amount));
        })
    };

    let on_get_balance = {
        let blockchain = blockchain.clone();
        let result = result.clone();
        Callback::from(move |id: String| {
            let chain = blockchain.borrow();
            result.set(chain.get_balance(&id));
        })
    };

    html! {
        <>
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
                        { (*result).clone() }
                    </div>

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
                                    let id = get_input_value("create-id");
                                    let balance = get_input_value("create-balance").parse().unwrap_or(0);
                                    on_create_account.emit((id, balance));
                                })
                            }>
                            { "Create" }
                        </button>
                    </div>

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
                                    on_transfer.emit((from, to, amount));
                                })
                            }>
                            { "Transfer" }
                        </button>
                    </div>

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
                                color:rgb(255, 255, 255);
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
                                    on_get_balance.emit(id);
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

fn get_input_value(id: &str) -> String {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
