use crate::api;
use crate::components::{MyProductCard, CreateProductForm};
use crate::types::Product;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

use yew::services::storage::Area;
use yew::services::{DialogService, StorageService};
struct State {
    products: Vec<Product>,
    get_products_error: Option<Error>,
    get_products_loaded: bool,
}

#[derive(Properties, Clone)]
pub struct Props {}

pub struct MyProducts {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    scene: Scene,
    storage: StorageService,
}

pub enum Msg {
    GetProducts,
    GetProductsSuccess(Vec<Product>),
    GetProductsError(Error),
    SwitchTo(Scene),
    AddProduct(Product),
    Publish(Product)
}

#[derive(Debug)]
pub enum Scene {
    ProductList,
    NewProductForm,
    Settings,
}

/// storage key for the products
const KEY: &str = "oma.anna.products";

impl Component for MyProducts {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let products = vec![];
        link.send_message(Msg::GetProducts);
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

        Self {
            props,
            state: State {
                products,
                get_products_error: None,
                get_products_loaded: false,
            },
            storage,
            link,
            task: None,
            scene: Scene::ProductList,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::SwitchTo(scene) => {
                self.scene = scene;
                true
            }
            Msg::AddProduct(product) => {
                self.state.products.push(product);
                self.storage.store(KEY, Json(&self.state.products));
                // we only need to re-render if we're currently displaying the clients
                matches!(self.scene, Scene::ProductList)
            }
            Msg::GetProducts => {
                self.state.get_products_loaded = false;
                let Json(products) = self.storage.restore(KEY);
                let products = products.ok().unwrap_or_else(Vec::new);
                log::info!("GetProducts::products {:?}", products);
                self.link.send_message(Msg::GetProductsSuccess(products));
                true
            }
            Msg::GetProductsSuccess(products) => {
                log::info!("GetProductsSuccess::products {:?}", products);

                self.state.products = products;
                self.state.get_products_loaded = true;
                true
            }
            Msg::GetProductsError(error) => {
                self.state.get_products_error = Some(error);
                self.state.get_products_loaded = true;
                true
            }
            Msg::Publish(product) => {
                
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                html! {

                        <MyProductCard product={product} />
                }
            })
            .collect();

        if !self.state.get_products_loaded {
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else if let Some(_) = self.state.get_products_error {
            html! {
                <div>
                    <span>{"Error loading products! :("}</span>
                </div>
            }
        } else {
            match self.scene {
                Scene::ProductList => html! {
                    <div class="crm">
                        <h1>{"List of products"}</h1>
                        <div class="product_card_list">{products}</div>

                        <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::NewProductForm))>{ "Add New" }</button>
                        <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::Settings))>{ "Settings" }</button>
                    </div>
                },
                Scene::NewProductForm => html! {
                    <div class="crm">
                        <h1>{"Add a new product"}</h1>
                        <CreateProductForm on_add=self.link.callback(Msg::AddProduct) on_abort=self.link.callback(|_| Msg::SwitchTo(Scene::ProductList)) />
                    </div>
                },
                Scene::Settings => html! {
                    <div>
                        <h1>{"Settings"}</h1>
                        // <button onclick=self.link.callback(|_| Msg::ClearProducts)>{ "Remove all clients" }</button>
                        <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::ProductList))>{ "Go Back" }</button>
                    </div>
                },
            }
        }
    }
}
