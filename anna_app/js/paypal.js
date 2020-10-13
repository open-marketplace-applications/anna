export function show_button(order) {
    console.log("hello world paypal!")
    console.log(order)
    console.log("hello world paypal!")
    window.paypal.Button.render({
        // Configure environment
        env: 'sandbox',
        client: {
          sandbox: 'ASdqo5xgUZOW_YrjgR0XnepLOLbJBCPhJK9N5TMVx0YRj1e3mM3jICi80CDorJZNtwnrC2uUQxSmFhTK',
          production: 'demo_production_client_id'
        },
        // Customize button (optional)
        locale: 'en_US',
        style: {
          size: 'small',
          color: 'gold',
          shape: 'pill',
        },
    
        // Enable Pay Now checkout flow (optional)
        commit: true,
    
        // Set up a payment
        payment: function(data, actions) {
          console.log("order")
          console.log(order)
          return actions.payment.create({
            transactions: [{
              amount: {
                total: order.final_price,
                currency: 'EUR'
              },
              invoice_number: order.id
            }]
          });
        },
        // Execute the payment
        onAuthorize: function(data, actions) {
          return actions.payment.execute().then(function(resp) {
            // Show a confirmation message to the buyer
            console.log("onAuthorize")
            console.log("data")
            console.log(data)
            console.log("actions")
            console.log(actions)
            console.log("resp")
            console.log(resp)
            window.wasm.handlePayPalPayment(resp)
            
            var field = document.getElementById("paypal-id");
            console.log(field);
            console.log("resp.id")
            console.log(resp.id)
            field.innerHTML = resp.id;


            var hangoutButton = document.getElementById("hangoutButtonId");
            console.log(hangoutButton)
            hangoutButton.click(); // this will trigger the click event

            // window.alert('Thank you for your purchase!');
          });
        },
        // onCancel
        onCancel: function(data, actions) {
          console.log("onCancel")

          window.alert('onCancel!');
        }
      }, '#paypal-button');
};