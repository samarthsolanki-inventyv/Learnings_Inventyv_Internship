/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Implement the functions needed to solve the exercise here.
 * Do not forget to export them so they are available for the
 * tests. Here an example of the syntax as reminder:
 *
 * export function yourFunction(...) {
 *   ...
 * }
 */

export function cookingStatus(time){
  if(time === 0){
    return 'Lasagna is done.'
  }
  if(time === undefined){
    return 'You forgot to set the timer.'
  }
  return  'Not done, please wait.'
}
export function preparationTime(layers, time = 2) {
  return layers.length * time;
}
export function quantities(layers){
  let noodles = 0;
  let sauce = 0;
  for (let index = 0; index <= layers.length; index++) {
    if(layers[index] === 'noodles') noodles += 50;
    if(layers[index] === 'sauce') sauce += 0.2;
   }
  return {
    noodles: noodles,
    sauce: sauce
  }
}
export function addSecretIngredient(friendsList, myList) {
  myList.push(friendsList.at(-1));
}
export function scaleRecipe(recipe, portions) {
  const scaledRecipe = {};
  const scaleFactor = portions / 2;
  
  for (const ingredient in recipe) {
    scaledRecipe[ingredient] = recipe[ingredient] * scaleFactor;
  }
  
  return scaledRecipe;
}

