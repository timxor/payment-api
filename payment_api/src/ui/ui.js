class UI {
  constructor(){
    this.tokens = ['ETH', 'OMG', 'DAI'];
    this.activeToken = '';
  }

  insertTokens (tks){
    const dropdown = document.querySelector('div.dropdown-menu');
    tks.forEach(tkn =>{
      const item = document.createElement('div');
      item.innerHTML = `
      <a href='#' id = ${tkn} class = 'dropdown-item'>${tkn}</a>
      `
      dropdown.appendChild(item);
    });
  }

  selectToken(e){
    let ddText = document.querySelector('button.btn.btn-outline-primary.dropdown-toggle')
    ddText.firstChild.nodeValue = e.target.id;
    this.activeToken = ddText.firstChild.nodeValue
    e.preventDefault();
    return this.activeToken;
  }

  throwAlert(message, type){
    const alertDiv = document.createElement('div');
    if(type==='error'){
      alertDiv.className = "alert alert-danger";
      alertDiv.appendChild(document.createTextNode(message));
    }

    else{
      alertDiv.className = "alert alert-success";
      alertDiv.appendChild(document.createTextNode(message));
    }

    const heading = document.querySelector('h1');
    const card = document.querySelector('.card');
    card.insertBefore(alertDiv, heading);
    setTimeout(function(){alertDiv.remove()},3000);

  }

  displayLoader(){
    const loader = document.getElementById('loading');
    loader.style.display = 'inline';
    setTimeout(function(){
      loader.remove();
      transferForm.remove();
      results.style.display = 'inline';
      },1000);
  }

}
