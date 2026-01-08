function pattern(n){
let i, j;
for (i = 1; i <= n; i++) {
  let row = "";
  for (j = 1; j <= n; j++) row += Math.min(i, j, n - i + 1, n - j + 1);
  console.log(row);
}
}

pattern(6)