/// <reference path="./global.d.ts" />
//
// @ts-check

/**
 * Determine the price of the pizza given the pizza and optional extras
 *
 * @param {Pizza} pizza name of the pizza to be made
 * @param {Extra[]} extras list of extras
 *
 * @returns {number} the price of the pizza
 */
export function pizzaPrice(pizza, ...extras) {
    const basePrices = {
      Margherita: 7,
      Caprese: 9,
      Formaggio: 10
    }
  const Extras = {
    ExtraSauce : 1,
    ExtraToppings: 2
  }
  let total = basePrices[pizza];
  extras.forEach(extra => {
    total += Extras[extra] || 0;
  });
  return total;
}

/**
 * Calculate the price of the total order, given individual orders
 *
 * (HINT: For this exercise, you can take a look at the supplied "global.d.ts" file
 * for a more info about the type definitions used)
 *
 * @param {PizzaOrder[]} pizzaOrders a list of pizza orders
 * @returns {number} the price of the total order
 */
export function orderPrice(pizzaOrders) {
  const basePrices = {
    Margherita: 7,
    Caprese: 9,
    Formaggio: 10,
  };

  let total = 0;

  for (let i = 0; i < pizzaOrders.length; i++) {
    const order = pizzaOrders[i];
    total += basePrices[order.pizza];
    for (let j = 0; j < order.extras.length; j++) {
      if (order.extras[j] === 'ExtraSauce') {
        total += 1;
      } else if (order.extras[j] === 'ExtraToppings') {
        total += 2;
      }
    }
  }

  return total;
}


