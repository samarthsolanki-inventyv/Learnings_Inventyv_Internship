function main(){
    var arr1 = [1,2,3,2,5,6];
    let removed = arr1.shift();
    console.log(arr1)
    console.log(removed)
      let finalarr = second(removed, arr1)
      return new Promise((resolve,reject) => {
        let sum = finalarr.reduce((accumulator, currentValue) => accumulator + currentValue, 0);
         if(sum < 45){
           resolve(sum);
         }else{
           reject("value excedeed 35")
         }
        
      }).then(result => {
        console.log("Resolved:", result);
      })
      .catch(error => {
        console.error("Rejected:", error);
      });
      
  }
   function second(removed,arr1){
     let  arr2 = [7,8,9];
      arr2 = [removed,...arr2,...arr1]
      console.log(arr2)
      return arr2;
  }
  main()
  