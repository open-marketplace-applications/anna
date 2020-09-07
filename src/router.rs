use crate::pages::{AppRoutes, Home, Profile, Products, ProductDetail};
// use std::marker::PhantomData;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::switch::Permissive;
use yew_router::{route::Route, router::Router as YewRouter};
use crate::types::{CartProduct, Product};

struct State {
    cart_products: Vec<CartProduct>,
}
pub struct Router {
    state: State,

    link: ComponentLink<Self>,
}

pub enum Msg {
    AddToCart(Product),
}


impl Component for Router {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let cart_products = vec![];

        Self {
            state: State { cart_products },
            link: _link,
        }    
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::AddToCart(product) => {
                log::info!("product: {:?}", product);
                
                let cart_product = self
                .state
                .cart_products
                .iter_mut()
                .find(|cp: &&mut CartProduct| cp.product.id == product.id);
                log::info!("cart_product: {:?}", cart_product);

                if let Some(cp) = cart_product {
                    cp.quantity += 1;
                } else {
                    self.state.cart_products.push(CartProduct {
                        product: product.clone(),
                        quantity: 1,
                    })
                }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let handle_add_to_cart = self.
            link
            .callback(|product: Product| Msg::AddToCart(product));
        let cart_products = self.state.cart_products.clone();

        html! {
            <YewRouter<AppRoutes>
                render= YewRouter::render(move |switch: AppRoutes| {
                    match switch {
                        AppRoutes::Home => html!{<Home />},
                        AppRoutes::Profile => html!{<Profile />},
                        AppRoutes::ProductDetail(id) => {
                            html! {<ProductDetail id=id on_add_to_cart=handle_add_to_cart.clone() />}
                        },
                        AppRoutes::Products => {
                            html! {
                                <Products cart_products=cart_products.clone() on_add_to_cart=handle_add_to_cart.clone()/>
                            }
                        },
                        AppRoutes::NotFound(Permissive(None)) => html!{"Page not found"},
                        AppRoutes::NotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                    }
                })
                redirect = YewRouter::redirect(|route: Route| {
                    AppRoutes::NotFound(Permissive(Some(route.route)))
                })
            />
        }
    }
}

// pub struct Protected<T: Component + 'static> {
//     props: T::Properties,
//     phantom: PhantomData<&'static T>,
// }

// impl<T> Component for Protected<T>
// where
//     T: Component + 'static,
// {
//     type Message = T::Message;
//     type Properties = T::Properties;

//     fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
//         Self {
//             props: props,
//             phantom: PhantomData,
//         }
//     }

//     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
//         true
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         true
//     }

//     fn view(&self) -> Html {
//         html! {<T />}
//     }
// }