/*
TO DO
0.2 multiple errors at once
2. insert callbacks to payment cleared sequence;; shouldn't need time delays
4. refresh page
*/



const transferForm = document.querySelector('input.btn.btn-danger.btn-block')
const dd = document.querySelector('div.dropdown-menu')
const tokens = ['ETH', 'OMG', 'DAI'];
const ui = new UI;
const results = document.getElementById('results');
const restartBtn = document.getElementById('restart');

ui.insertTokens(ui.tokens);
let activeToken = '';

dd.addEventListener('click',selectToken);
transferForm.addEventListener('click',handleTx);
restartBtn.addEventListener('click',restart);

/* HANDLE EVENTS */
function handleTx(e){
  const tx = new Transaction;
  tx.takeTx();
  e.preventDefault();
}


function selectToken(e){
  ui.selectToken(e);
  e.preventDefault();
}

function restart(){
  console.log('restarting');
  location.reload();
}
