class Transaction {
  constructor(){
    this.amount = 0;
    this.token = '';
    this.name = '';
    this.phone = '';
    this.email = '';
  }
  takeTx(){
    this.amount = document.getElementById('amount2').value
    if (ui.activeToken === ''){
      this.token = '';
    }
    else{
      this.token = ui.activeToken;
    }
    this.name = document.getElementById('name').value;
    this.phone = document.getElementById('number').value;
    this.email = document.getElementById('email').value;
    this.checkForErrors(this.amount, this.token, this.name, this.phone, this.email);
  }

  checkForErrors(amount, token, name, number, email){
    if(!this.amount){
      ui.throwAlert('Please insert an amount of crypto', 'error');
    }
    else if(!this.token){
      ui.throwAlert('Please select a token', 'error');
    }
    else if(!this.name){
      ui.throwAlert('Please provide a name for the recipient', 'error');
    }
    else if(!this.phone){
      ui.throwAlert('Please provide a phone number for the recipient', 'error');
    }
    else if(!this.email){
      ui.throwAlert('Please provide an email address for the recipient', 'error');
    } // figure out how to throw multiple at once
    else{
      this.doTx(this.amount, this.token, this.name, this.phone, this.email);
    }
  }

  doTx(amount, token, name, number, email){
    const pmtAmt = document.getElementById('pmt-amt');
    pmtAmt.value = `${amount} ${token} sent`
    const recipient = document.getElementById('recipient');
    recipient.value = `${name} ${number} ${email}`;
    ui.displayLoader(),
    setTimeout(ui.throwAlert('Payment cleared yo!', 'success'),1000); // write as a callback

  }
}
