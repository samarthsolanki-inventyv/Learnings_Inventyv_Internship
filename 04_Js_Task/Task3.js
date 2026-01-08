function main() {
    const ARR1 = Symbol("arr1");
    const ARR2 = Symbol("arr2");
  
    const store = {
      [ARR1]: [1, 2, 4, 4, 4, 6]
    };
  
    let removed = store[ARR1].shift();
  
    console.log(store[ARR1]);
    console.log(removed);
  
    const store1 = {
      [ARR2]: second(removed, store[ARR1])
    };
  
    return new Promise((resolve, reject) => {
      let sum = store1[ARR2].reduce(
        (accumulator, currentValue) => accumulator + currentValue,
        0
      );
  
      if (sum < 35) {
        resolve(sum);
      } else {
        reject("value exceeded 35");
      }
    })
      .then(result => {
        console.log("Resolved:", result);
      })
      .catch(error => {
        console.error("Rejected:", error);
      });
  }
  
  function second(removed, arr1) {
    return [removed, 7, 8, 9, ...arr1];
  }
  
  main();
  