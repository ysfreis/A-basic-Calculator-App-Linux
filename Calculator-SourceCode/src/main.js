const { invoke } = window.__TAURI__.tauri;


let fs;
let o;
let ls;
let sonuc;

async function hesapla() {
  if(fs.value.length == 0 || ls.value.length==0){
    sonuc.textContent = "lütfen boşlukları doldurunuz";

  }else{
    sonuc.textContent = await invoke("hesapla",{ a : fs.value,b:o.value,c:ls.value});
    console.log("gönderildi");
  }
}
window.addEventListener("DOMContentLoaded", () => {
  
  sonuc = document.querySelector("#sonuc");
  
  fs = document.querySelector("#fs");
  o = document.querySelector("#o");
  ls = document.querySelector("#ls");
  document.querySelector("#m").addEventListener("submit", (e) => {
    e.preventDefault();
    hesapla();
  });
});
